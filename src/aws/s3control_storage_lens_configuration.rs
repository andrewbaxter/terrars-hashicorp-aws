#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3controlStorageLensConfigurationData_ {
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
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    config_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_lens_configuration: std
    ::option
    ::Option<std::vec::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration>>,
    dynamic: S3controlStorageLensConfigurationDynamic,
}

struct S3controlStorageLensConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3controlStorageLensConfigurationData_>,
}

#[derive(Clone)]
pub struct S3controlStorageLensConfiguration(std::rc::Rc<S3controlStorageLensConfiguration_>);

impl S3controlStorageLensConfiguration {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `storage_lens_configuration`.\n"]
    pub fn set_storage_lens_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<S3controlStorageLensConfiguration_StorageLensConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().storage_lens_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.storage_lens_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_id` after provisioning.\n"]
    pub fn config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_lens_configuration` after provisioning.\n"]
    pub fn storage_lens_configuration(
        &self,
    ) -> terrars::ListRef<S3controlStorageLensConfiguration_StorageLensConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_lens_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for S3controlStorageLensConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3controlStorageLensConfiguration { }

impl terrars::ToListMappable for S3controlStorageLensConfiguration {
    type O = terrars::ListRef<S3controlStorageLensConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3controlStorageLensConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3control_storage_lens_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3controlStorageLensConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub config_id: terrars::PrimField<std::string::String>,
}

impl BuildS3controlStorageLensConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> S3controlStorageLensConfiguration {
        let out = S3controlStorageLensConfiguration(std::rc::Rc::new(S3controlStorageLensConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3controlStorageLensConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: core::default::Default::default(),
                config_id: self.config_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                storage_lens_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3controlStorageLensConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlStorageLensConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3controlStorageLensConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_id` after provisioning.\n"]
    pub fn config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_lens_configuration` after provisioning.\n"]
    pub fn storage_lens_configuration(
        &self,
    ) -> terrars::ListRef<S3controlStorageLensConfiguration_StorageLensConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_lens_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfigurationDynamic {
    storage_lens_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<S3controlStorageLensConfiguration_StorageLensConfiguration>>,
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetrics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetrics {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetrics {
    type O =
        terrars
        ::BlockAssignable<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetrics {}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetrics {
    pub fn build(self) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetrics {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetrics {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetricsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetricsRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetrics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetrics {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetrics {
    type O =
        terrars
        ::BlockAssignable<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetrics,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetrics {}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetrics {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetrics {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetrics {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetricsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetricsRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetrics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetrics {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetrics {
    type O =
        terrars
        ::BlockAssignable<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetrics,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetrics {}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetrics {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetrics {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetrics {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetricsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetricsRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetrics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetrics {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetrics {
    type O =
        terrars
        ::BlockAssignable<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetrics,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetrics {}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetrics {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetrics {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetrics {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetricsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetricsRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetrics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetrics {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetrics {
    type O =
        terrars
        ::BlockAssignable<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetrics,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetrics {}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetrics {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetrics {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetrics {
            enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetricsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetricsRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics_SelectionCriteria {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_depth: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_storage_bytes_percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics_SelectionCriteria {
    #[doc = "Set the field `delimiter`.\n"]
    pub fn set_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_depth`.\n"]
    pub fn set_max_depth(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_depth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_storage_bytes_percentage`.\n"]
    pub fn set_min_storage_bytes_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_storage_bytes_percentage = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics_SelectionCriteria {
    type O =
        terrars
        ::BlockAssignable<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics_SelectionCriteria,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics_SelectionCriteria {}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics_SelectionCriteria {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics_SelectionCriteria {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics_SelectionCriteria {
            delimiter: core::default::Default::default(),
            max_depth: core::default::Default::default(),
            min_storage_bytes_percentage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics_SelectionCriteriaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics_SelectionCriteriaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics_SelectionCriteriaRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics_SelectionCriteriaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics_SelectionCriteriaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delimiter` after provisioning.\n"]
    pub fn delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `max_depth` after provisioning.\n"]
    pub fn max_depth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_depth", self.base))
    }

    #[doc = "Get a reference to the value of field `min_storage_bytes_percentage` after provisioning.\n"]
    pub fn min_storage_bytes_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_storage_bytes_percentage", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetricsDynamic {
    selection_criteria: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics_SelectionCriteria,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selection_criteria: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics_SelectionCriteria,
        >,
    >,
    dynamic: S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetricsDynamic,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `selection_criteria`.\n"]
    pub fn set_selection_criteria(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics_SelectionCriteria,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.selection_criteria = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.selection_criteria = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics {
    type O =
        terrars
        ::BlockAssignable<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics {}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics {
            enabled: core::default::Default::default(),
            selection_criteria: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetricsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetricsRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `selection_criteria` after provisioning.\n"]
    pub fn selection_criteria(
        &self,
    ) -> terrars
    ::ListRef<
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics_SelectionCriteriaRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.selection_criteria", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevelDynamic {
    storage_metrics: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_metrics: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics,
        >,
    >,
    dynamic: S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevelDynamic,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel {
    #[doc = "Set the field `storage_metrics`.\n"]
    pub fn set_storage_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetrics,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage_metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_metrics = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel {
    type O =
        terrars
        ::BlockAssignable<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel {}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel {
            storage_metrics: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevelRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevelRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `storage_metrics` after provisioning.\n"]
    pub fn storage_metrics(
        &self,
    ) -> terrars
    ::ListRef<
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel_StorageMetricsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_metrics", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevelDynamic {
    activity_metrics: std
    ::option
    ::Option<
        terrars::DynamicBlock<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetrics>,
    >,
    advanced_cost_optimization_metrics: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetrics,
        >,
    >,
    advanced_data_protection_metrics: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetrics,
        >,
    >,
    advanced_performance_metrics: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetrics,
        >,
    >,
    detailed_status_code_metrics: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetrics,
        >,
    >,
    prefix_level: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    activity_metrics: std
    ::option
    ::Option<std::vec::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetrics>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_cost_optimization_metrics: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetrics>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_data_protection_metrics: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetrics>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_performance_metrics: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetrics>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detailed_status_code_metrics: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetrics>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_level: std
    ::option
    ::Option<
        std::vec::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel>,
    >,
    dynamic: S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevelDynamic,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel {
    #[doc = "Set the field `activity_metrics`.\n"]
    pub fn set_activity_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetrics,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.activity_metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.activity_metrics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `advanced_cost_optimization_metrics`.\n"]
    pub fn set_advanced_cost_optimization_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetrics,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.advanced_cost_optimization_metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_cost_optimization_metrics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `advanced_data_protection_metrics`.\n"]
    pub fn set_advanced_data_protection_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetrics,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.advanced_data_protection_metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_data_protection_metrics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `advanced_performance_metrics`.\n"]
    pub fn set_advanced_performance_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetrics,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.advanced_performance_metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_performance_metrics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `detailed_status_code_metrics`.\n"]
    pub fn set_detailed_status_code_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetrics,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.detailed_status_code_metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.detailed_status_code_metrics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `prefix_level`.\n"]
    pub fn set_prefix_level(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevel,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.prefix_level = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.prefix_level = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel {
    type O =
        terrars
        ::BlockAssignable<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel {}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel {
    pub fn build(self) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel {
            activity_metrics: core::default::Default::default(),
            advanced_cost_optimization_metrics: core::default::Default::default(),
            advanced_data_protection_metrics: core::default::Default::default(),
            advanced_performance_metrics: core::default::Default::default(),
            detailed_status_code_metrics: core::default::Default::default(),
            prefix_level: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevelRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevelRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `activity_metrics` after provisioning.\n"]
    pub fn activity_metrics(
        &self,
    ) -> terrars::ListRef<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.activity_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_cost_optimization_metrics` after provisioning.\n"]
    pub fn advanced_cost_optimization_metrics(
        &self,
    ) -> terrars
    ::ListRef<
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetricsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_cost_optimization_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_data_protection_metrics` after provisioning.\n"]
    pub fn advanced_data_protection_metrics(
        &self,
    ) -> terrars
    ::ListRef<
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetricsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_data_protection_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_performance_metrics` after provisioning.\n"]
    pub fn advanced_performance_metrics(
        &self,
    ) -> terrars
    ::ListRef<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_performance_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `detailed_status_code_metrics` after provisioning.\n"]
    pub fn detailed_status_code_metrics(
        &self,
    ) -> terrars
    ::ListRef<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.detailed_status_code_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_level` after provisioning.\n"]
    pub fn prefix_level(
        &self,
    ) -> terrars
    ::ListRef<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel_PrefixLevelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prefix_level", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevelDynamic {
    activity_metrics: std
    ::option
    ::Option<
        terrars::DynamicBlock<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetrics>,
    >,
    advanced_cost_optimization_metrics: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetrics,
        >,
    >,
    advanced_data_protection_metrics: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetrics,
        >,
    >,
    advanced_performance_metrics: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetrics,
        >,
    >,
    bucket_level: std
    ::option
    ::Option<
        terrars::DynamicBlock<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel>,
    >,
    detailed_status_code_metrics: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetrics,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    activity_metrics: std
    ::option
    ::Option<std::vec::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetrics>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_cost_optimization_metrics: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetrics>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_data_protection_metrics: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetrics>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_performance_metrics: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetrics>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_level: std
    ::option
    ::Option<std::vec::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    detailed_status_code_metrics: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetrics>,
    >,
    dynamic: S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevelDynamic,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel {
    #[doc = "Set the field `activity_metrics`.\n"]
    pub fn set_activity_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetrics,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.activity_metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.activity_metrics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `advanced_cost_optimization_metrics`.\n"]
    pub fn set_advanced_cost_optimization_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetrics,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.advanced_cost_optimization_metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_cost_optimization_metrics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `advanced_data_protection_metrics`.\n"]
    pub fn set_advanced_data_protection_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetrics,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.advanced_data_protection_metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_data_protection_metrics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `advanced_performance_metrics`.\n"]
    pub fn set_advanced_performance_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetrics,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.advanced_performance_metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.advanced_performance_metrics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bucket_level`.\n"]
    pub fn set_bucket_level(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevel,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bucket_level = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bucket_level = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `detailed_status_code_metrics`.\n"]
    pub fn set_detailed_status_code_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetrics,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.detailed_status_code_metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.detailed_status_code_metrics = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel {
    type O = terrars::BlockAssignable<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel {}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel {
    pub fn build(self) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel {
            activity_metrics: core::default::Default::default(),
            advanced_cost_optimization_metrics: core::default::Default::default(),
            advanced_data_protection_metrics: core::default::Default::default(),
            advanced_performance_metrics: core::default::Default::default(),
            bucket_level: core::default::Default::default(),
            detailed_status_code_metrics: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevelRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevelRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `activity_metrics` after provisioning.\n"]
    pub fn activity_metrics(
        &self,
    ) -> terrars::ListRef<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_ActivityMetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.activity_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_cost_optimization_metrics` after provisioning.\n"]
    pub fn advanced_cost_optimization_metrics(
        &self,
    ) -> terrars
    ::ListRef<
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedCostOptimizationMetricsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_cost_optimization_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_data_protection_metrics` after provisioning.\n"]
    pub fn advanced_data_protection_metrics(
        &self,
    ) -> terrars
    ::ListRef<
        S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedDataProtectionMetricsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_data_protection_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `advanced_performance_metrics` after provisioning.\n"]
    pub fn advanced_performance_metrics(
        &self,
    ) -> terrars
    ::ListRef<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_AdvancedPerformanceMetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_performance_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_level` after provisioning.\n"]
    pub fn bucket_level(
        &self,
    ) -> terrars::ListRef<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_BucketLevelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bucket_level", self.base))
    }

    #[doc = "Get a reference to the value of field `detailed_status_code_metrics` after provisioning.\n"]
    pub fn detailed_status_code_metrics(
        &self,
    ) -> terrars
    ::ListRef<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel_DetailedStatusCodeMetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.detailed_status_code_metrics", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AwsOrg {
    arn: terrars::PrimField<std::string::String>,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AwsOrg { }

impl terrars::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_AwsOrg {
    type O = terrars::BlockAssignable<S3controlStorageLensConfiguration_StorageLensConfiguration_AwsOrg>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AwsOrg {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_AwsOrg {
    pub fn build(self) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AwsOrg {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AwsOrg { arn: self.arn }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_AwsOrgRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_AwsOrgRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_AwsOrgRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_AwsOrgRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_AwsOrgRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_CloudWatchMetrics {
    enabled: terrars::PrimField<bool>,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_CloudWatchMetrics { }

impl terrars
::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_CloudWatchMetrics {
    type O =
        terrars
        ::BlockAssignable<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_CloudWatchMetrics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_CloudWatchMetrics {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_CloudWatchMetrics {
    pub fn build(self) -> S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_CloudWatchMetrics {
        S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_CloudWatchMetrics {
            enabled: self.enabled,
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_CloudWatchMetricsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_CloudWatchMetricsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_CloudWatchMetricsRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_CloudWatchMetricsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_CloudWatchMetricsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseKms {
    key_id: terrars::PrimField<std::string::String>,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseKms { }

impl terrars
::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseKms {
    type O =
        terrars
        ::BlockAssignable<
            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseKms,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseKms {
    #[doc = ""]
    pub key_id: terrars::PrimField<std::string::String>,
}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseKms {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseKms {
        S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseKms {
            key_id: self.key_id,
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseKmsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseKmsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseKmsRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseKmsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseKmsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseS3 {}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseS3 { }

impl terrars
::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseS3 {
    type O =
        terrars
        ::BlockAssignable<
            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseS3,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseS3 {}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseS3 {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseS3 {
        S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseS3 {}
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseS3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseS3Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseS3Ref {
        S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseS3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseS3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_EncryptionDynamic {
    sse_kms: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseKms,
        >,
    >,
    sse_s3: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseS3,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sse_kms: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseKms,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sse_s3: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseS3,
        >,
    >,
    dynamic: S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_EncryptionDynamic,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption {
    #[doc = "Set the field `sse_kms`.\n"]
    pub fn set_sse_kms(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseKms,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sse_kms = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sse_kms = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sse_s3`.\n"]
    pub fn set_sse_s3(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseS3,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sse_s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sse_s3 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption {
    type O =
        terrars
        ::BlockAssignable<
            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption {}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption {
        S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption {
            sse_kms: core::default::Default::default(),
            sse_s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_EncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_EncryptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_EncryptionRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_EncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_EncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sse_kms` after provisioning.\n"]
    pub fn sse_kms(
        &self,
    ) -> terrars
    ::ListRef<
        S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseKmsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sse_kms", self.base))
    }

    #[doc = "Get a reference to the value of field `sse_s3` after provisioning.\n"]
    pub fn sse_s3(
        &self,
    ) -> terrars
    ::ListRef<
        S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption_SseS3Ref,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sse_s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestinationDynamic {
    encryption: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination {
    account_id: terrars::PrimField<std::string::String>,
    arn: terrars::PrimField<std::string::String>,
    format: terrars::PrimField<std::string::String>,
    output_schema_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption>,
    >,
    dynamic: S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestinationDynamic,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination {
    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption`.\n"]
    pub fn set_encryption(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination {
    type O =
        terrars
        ::BlockAssignable<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination {
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub format: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub output_schema_version: terrars::PrimField<std::string::String>,
}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination {
    pub fn build(self) -> S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination {
        S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination {
            account_id: self.account_id,
            arn: self.arn,
            format: self.format,
            output_schema_version: self.output_schema_version,
            prefix: core::default::Default::default(),
            encryption: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestinationRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.base))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `format` after provisioning.\n"]
    pub fn format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.format", self.base))
    }

    #[doc = "Get a reference to the value of field `output_schema_version` after provisioning.\n"]
    pub fn output_schema_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.output_schema_version", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption` after provisioning.\n"]
    pub fn encryption(
        &self,
    ) -> terrars
    ::ListRef<
        S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_EncryptionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestinationDynamic {
    encryption: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestination {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption>,
    >,
    dynamic: S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestinationDynamic,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestination {
    #[doc = "Set the field `encryption`.\n"]
    pub fn set_encryption(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_Encryption,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestination {
    type O =
        terrars
        ::BlockAssignable<
            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestination,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestination {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestination {
    pub fn build(
        self,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestination {
        S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestination {
            enabled: self.enabled,
            encryption: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestinationRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption` after provisioning.\n"]
    pub fn encryption(
        &self,
    ) -> terrars
    ::ListRef<
        S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination_EncryptionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfiguration_StorageLensConfiguration_DataExportDynamic {
    cloud_watch_metrics: std
    ::option
    ::Option<
        terrars::DynamicBlock<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_CloudWatchMetrics>,
    >,
    s3_bucket_destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination>,
    >,
    storage_lens_table_destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestination,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloud_watch_metrics: std
    ::option
    ::Option<std::vec::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_CloudWatchMetrics>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket_destination: std
    ::option
    ::Option<
        std::vec::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_lens_table_destination: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestination>,
    >,
    dynamic: S3controlStorageLensConfiguration_StorageLensConfiguration_DataExportDynamic,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport {
    #[doc = "Set the field `cloud_watch_metrics`.\n"]
    pub fn set_cloud_watch_metrics(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_CloudWatchMetrics,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloud_watch_metrics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloud_watch_metrics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_bucket_destination`.\n"]
    pub fn set_s3_bucket_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_bucket_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_bucket_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_lens_table_destination`.\n"]
    pub fn set_storage_lens_table_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestination,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage_lens_table_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_lens_table_destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport {
    type O = terrars::BlockAssignable<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_DataExport {}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_DataExport {
    pub fn build(self) -> S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport {
        S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport {
            cloud_watch_metrics: core::default::Default::default(),
            s3_bucket_destination: core::default::Default::default(),
            storage_lens_table_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_DataExportRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_DataExportRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_DataExportRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_DataExportRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_DataExportRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_watch_metrics` after provisioning.\n"]
    pub fn cloud_watch_metrics(
        &self,
    ) -> terrars::ListRef<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_CloudWatchMetricsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloud_watch_metrics", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_destination` after provisioning.\n"]
    pub fn s3_bucket_destination(
        &self,
    ) -> terrars
    ::ListRef<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_bucket_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_lens_table_destination` after provisioning.\n"]
    pub fn storage_lens_table_destination(
        &self,
    ) -> terrars
    ::ListRef<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_lens_table_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_Exclude {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buckets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_Exclude {
    #[doc = "Set the field `buckets`.\n"]
    pub fn set_buckets(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.buckets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.regions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_Exclude {
    type O = terrars::BlockAssignable<S3controlStorageLensConfiguration_StorageLensConfiguration_Exclude>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_Exclude {}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_Exclude {
    pub fn build(self) -> S3controlStorageLensConfiguration_StorageLensConfiguration_Exclude {
        S3controlStorageLensConfiguration_StorageLensConfiguration_Exclude {
            buckets: core::default::Default::default(),
            regions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_ExcludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_ExcludeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_ExcludeRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_ExcludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_ExcludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `buckets` after provisioning.\n"]
    pub fn buckets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfiguration_StorageLensConfiguration_ExpandedPrefixesDataExportDynamic {
    s3_bucket_destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination>,
    >,
    storage_lens_table_destination: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestination,
        >,
    >,
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_ExpandedPrefixesDataExport {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket_destination: std
    ::option
    ::Option<
        std::vec::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_lens_table_destination: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestination>,
    >,
    dynamic: S3controlStorageLensConfiguration_StorageLensConfiguration_ExpandedPrefixesDataExportDynamic,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_ExpandedPrefixesDataExport {
    #[doc = "Set the field `s3_bucket_destination`.\n"]
    pub fn set_s3_bucket_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestination,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_bucket_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_bucket_destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_lens_table_destination`.\n"]
    pub fn set_storage_lens_table_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestination,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage_lens_table_destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_lens_table_destination = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_ExpandedPrefixesDataExport {
    type O =
        terrars
        ::BlockAssignable<S3controlStorageLensConfiguration_StorageLensConfiguration_ExpandedPrefixesDataExport>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_ExpandedPrefixesDataExport {}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_ExpandedPrefixesDataExport {
    pub fn build(self) -> S3controlStorageLensConfiguration_StorageLensConfiguration_ExpandedPrefixesDataExport {
        S3controlStorageLensConfiguration_StorageLensConfiguration_ExpandedPrefixesDataExport {
            s3_bucket_destination: core::default::Default::default(),
            storage_lens_table_destination: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_ExpandedPrefixesDataExportRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_ExpandedPrefixesDataExportRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_ExpandedPrefixesDataExportRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_ExpandedPrefixesDataExportRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_ExpandedPrefixesDataExportRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_bucket_destination` after provisioning.\n"]
    pub fn s3_bucket_destination(
        &self,
    ) -> terrars
    ::ListRef<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_S3BucketDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_bucket_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_lens_table_destination` after provisioning.\n"]
    pub fn storage_lens_table_destination(
        &self,
    ) -> terrars
    ::ListRef<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport_StorageLensTableDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_lens_table_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_Include {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buckets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_Include {
    #[doc = "Set the field `buckets`.\n"]
    pub fn set_buckets(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.buckets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.regions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration_Include {
    type O = terrars::BlockAssignable<S3controlStorageLensConfiguration_StorageLensConfiguration_Include>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration_Include {}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration_Include {
    pub fn build(self) -> S3controlStorageLensConfiguration_StorageLensConfiguration_Include {
        S3controlStorageLensConfiguration_StorageLensConfiguration_Include {
            buckets: core::default::Default::default(),
            regions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration_IncludeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlStorageLensConfiguration_StorageLensConfiguration_IncludeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfiguration_IncludeRef {
        S3controlStorageLensConfiguration_StorageLensConfiguration_IncludeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration_IncludeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `buckets` after provisioning.\n"]
    pub fn buckets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.buckets", self.base))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3controlStorageLensConfiguration_StorageLensConfigurationDynamic {
    account_level: std
    ::option
    ::Option<terrars::DynamicBlock<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel>>,
    aws_org: std
    ::option
    ::Option<terrars::DynamicBlock<S3controlStorageLensConfiguration_StorageLensConfiguration_AwsOrg>>,
    data_export: std
    ::option
    ::Option<terrars::DynamicBlock<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport>>,
    exclude: std
    ::option
    ::Option<terrars::DynamicBlock<S3controlStorageLensConfiguration_StorageLensConfiguration_Exclude>>,
    expanded_prefixes_data_export: std
    ::option
    ::Option<
        terrars::DynamicBlock<S3controlStorageLensConfiguration_StorageLensConfiguration_ExpandedPrefixesDataExport>,
    >,
    include: std
    ::option
    ::Option<terrars::DynamicBlock<S3controlStorageLensConfiguration_StorageLensConfiguration_Include>>,
}

#[derive(Serialize)]
pub struct S3controlStorageLensConfiguration_StorageLensConfiguration {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_delimiter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_level: std
    ::option
    ::Option<std::vec::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_org: std::option::Option<std::vec::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_AwsOrg>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_export: std
    ::option
    ::Option<std::vec::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude: std::option::Option<std::vec::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_Exclude>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expanded_prefixes_data_export: std
    ::option
    ::Option<std::vec::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_ExpandedPrefixesDataExport>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include: std::option::Option<std::vec::Vec<S3controlStorageLensConfiguration_StorageLensConfiguration_Include>>,
    dynamic: S3controlStorageLensConfiguration_StorageLensConfigurationDynamic,
}

impl S3controlStorageLensConfiguration_StorageLensConfiguration {
    #[doc = "Set the field `prefix_delimiter`.\n"]
    pub fn set_prefix_delimiter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_delimiter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `account_level`.\n"]
    pub fn set_account_level(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevel>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.account_level = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.account_level = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aws_org`.\n"]
    pub fn set_aws_org(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<S3controlStorageLensConfiguration_StorageLensConfiguration_AwsOrg>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_org = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_org = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_export`.\n"]
    pub fn set_data_export(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExport>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_export = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_export = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exclude`.\n"]
    pub fn set_exclude(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<S3controlStorageLensConfiguration_StorageLensConfiguration_Exclude>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclude = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclude = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `expanded_prefixes_data_export`.\n"]
    pub fn set_expanded_prefixes_data_export(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            S3controlStorageLensConfiguration_StorageLensConfiguration_ExpandedPrefixesDataExport,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.expanded_prefixes_data_export = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.expanded_prefixes_data_export = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `include`.\n"]
    pub fn set_include(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<S3controlStorageLensConfiguration_StorageLensConfiguration_Include>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.include = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.include = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3controlStorageLensConfiguration_StorageLensConfiguration {
    type O = terrars::BlockAssignable<S3controlStorageLensConfiguration_StorageLensConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3controlStorageLensConfiguration_StorageLensConfiguration {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildS3controlStorageLensConfiguration_StorageLensConfiguration {
    pub fn build(self) -> S3controlStorageLensConfiguration_StorageLensConfiguration {
        S3controlStorageLensConfiguration_StorageLensConfiguration {
            enabled: self.enabled,
            prefix_delimiter: core::default::Default::default(),
            account_level: core::default::Default::default(),
            aws_org: core::default::Default::default(),
            data_export: core::default::Default::default(),
            exclude: core::default::Default::default(),
            expanded_prefixes_data_export: core::default::Default::default(),
            include: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3controlStorageLensConfiguration_StorageLensConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3controlStorageLensConfiguration_StorageLensConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3controlStorageLensConfiguration_StorageLensConfigurationRef {
        S3controlStorageLensConfiguration_StorageLensConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3controlStorageLensConfiguration_StorageLensConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_delimiter` after provisioning.\n"]
    pub fn prefix_delimiter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_delimiter", self.base))
    }

    #[doc = "Get a reference to the value of field `account_level` after provisioning.\n"]
    pub fn account_level(
        &self,
    ) -> terrars::ListRef<S3controlStorageLensConfiguration_StorageLensConfiguration_AccountLevelRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.account_level", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_org` after provisioning.\n"]
    pub fn aws_org(&self) -> terrars::ListRef<S3controlStorageLensConfiguration_StorageLensConfiguration_AwsOrgRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_org", self.base))
    }

    #[doc = "Get a reference to the value of field `data_export` after provisioning.\n"]
    pub fn data_export(
        &self,
    ) -> terrars::ListRef<S3controlStorageLensConfiguration_StorageLensConfiguration_DataExportRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_export", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude` after provisioning.\n"]
    pub fn exclude(&self) -> terrars::ListRef<S3controlStorageLensConfiguration_StorageLensConfiguration_ExcludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude", self.base))
    }

    #[doc = "Get a reference to the value of field `expanded_prefixes_data_export` after provisioning.\n"]
    pub fn expanded_prefixes_data_export(
        &self,
    ) -> terrars::ListRef<S3controlStorageLensConfiguration_StorageLensConfiguration_ExpandedPrefixesDataExportRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expanded_prefixes_data_export", self.base))
    }

    #[doc = "Get a reference to the value of field `include` after provisioning.\n"]
    pub fn include(&self) -> terrars::ListRef<S3controlStorageLensConfiguration_StorageLensConfiguration_IncludeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.include", self.base))
    }
}
