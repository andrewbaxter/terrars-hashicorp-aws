#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlueSecurityConfigurationData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std::option::Option<std::vec::Vec<GlueSecurityConfiguration_EncryptionConfiguration>>,
    dynamic: GlueSecurityConfigurationDynamic,
}

struct GlueSecurityConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GlueSecurityConfigurationData_>,
}

#[derive(Clone)]
pub struct GlueSecurityConfiguration(std::rc::Rc<GlueSecurityConfiguration_>);

impl GlueSecurityConfiguration {
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

    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueSecurityConfiguration_EncryptionConfiguration>>,
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<GlueSecurityConfiguration_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for GlueSecurityConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GlueSecurityConfiguration { }

impl terrars::ToListMappable for GlueSecurityConfiguration {
    type O = terrars::ListRef<GlueSecurityConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GlueSecurityConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_glue_security_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlueSecurityConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGlueSecurityConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> GlueSecurityConfiguration {
        let out = GlueSecurityConfiguration(std::rc::Rc::new(GlueSecurityConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GlueSecurityConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlueSecurityConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueSecurityConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GlueSecurityConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<GlueSecurityConfiguration_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct GlueSecurityConfigurationDynamic {
    encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<GlueSecurityConfiguration_EncryptionConfiguration>>,
}

#[derive(Serialize)]
pub struct GlueSecurityConfiguration_EncryptionConfiguration_CloudwatchEncryption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_encryption_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueSecurityConfiguration_EncryptionConfiguration_CloudwatchEncryption {
    #[doc = "Set the field `cloudwatch_encryption_mode`.\n"]
    pub fn set_cloudwatch_encryption_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cloudwatch_encryption_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueSecurityConfiguration_EncryptionConfiguration_CloudwatchEncryption {
    type O = terrars::BlockAssignable<GlueSecurityConfiguration_EncryptionConfiguration_CloudwatchEncryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueSecurityConfiguration_EncryptionConfiguration_CloudwatchEncryption {}

impl BuildGlueSecurityConfiguration_EncryptionConfiguration_CloudwatchEncryption {
    pub fn build(self) -> GlueSecurityConfiguration_EncryptionConfiguration_CloudwatchEncryption {
        GlueSecurityConfiguration_EncryptionConfiguration_CloudwatchEncryption {
            cloudwatch_encryption_mode: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueSecurityConfiguration_EncryptionConfiguration_CloudwatchEncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueSecurityConfiguration_EncryptionConfiguration_CloudwatchEncryptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueSecurityConfiguration_EncryptionConfiguration_CloudwatchEncryptionRef {
        GlueSecurityConfiguration_EncryptionConfiguration_CloudwatchEncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueSecurityConfiguration_EncryptionConfiguration_CloudwatchEncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_encryption_mode` after provisioning.\n"]
    pub fn cloudwatch_encryption_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_encryption_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueSecurityConfiguration_EncryptionConfiguration_JobBookmarksEncryption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_bookmarks_encryption_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueSecurityConfiguration_EncryptionConfiguration_JobBookmarksEncryption {
    #[doc = "Set the field `job_bookmarks_encryption_mode`.\n"]
    pub fn set_job_bookmarks_encryption_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.job_bookmarks_encryption_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueSecurityConfiguration_EncryptionConfiguration_JobBookmarksEncryption {
    type O = terrars::BlockAssignable<GlueSecurityConfiguration_EncryptionConfiguration_JobBookmarksEncryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueSecurityConfiguration_EncryptionConfiguration_JobBookmarksEncryption {}

impl BuildGlueSecurityConfiguration_EncryptionConfiguration_JobBookmarksEncryption {
    pub fn build(self) -> GlueSecurityConfiguration_EncryptionConfiguration_JobBookmarksEncryption {
        GlueSecurityConfiguration_EncryptionConfiguration_JobBookmarksEncryption {
            job_bookmarks_encryption_mode: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueSecurityConfiguration_EncryptionConfiguration_JobBookmarksEncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueSecurityConfiguration_EncryptionConfiguration_JobBookmarksEncryptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueSecurityConfiguration_EncryptionConfiguration_JobBookmarksEncryptionRef {
        GlueSecurityConfiguration_EncryptionConfiguration_JobBookmarksEncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueSecurityConfiguration_EncryptionConfiguration_JobBookmarksEncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `job_bookmarks_encryption_mode` after provisioning.\n"]
    pub fn job_bookmarks_encryption_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.job_bookmarks_encryption_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueSecurityConfiguration_EncryptionConfiguration_S3Encryption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_encryption_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueSecurityConfiguration_EncryptionConfiguration_S3Encryption {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_encryption_mode`.\n"]
    pub fn set_s3_encryption_mode(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.s3_encryption_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueSecurityConfiguration_EncryptionConfiguration_S3Encryption {
    type O = terrars::BlockAssignable<GlueSecurityConfiguration_EncryptionConfiguration_S3Encryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueSecurityConfiguration_EncryptionConfiguration_S3Encryption {}

impl BuildGlueSecurityConfiguration_EncryptionConfiguration_S3Encryption {
    pub fn build(self) -> GlueSecurityConfiguration_EncryptionConfiguration_S3Encryption {
        GlueSecurityConfiguration_EncryptionConfiguration_S3Encryption {
            kms_key_arn: core::default::Default::default(),
            s3_encryption_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueSecurityConfiguration_EncryptionConfiguration_S3EncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueSecurityConfiguration_EncryptionConfiguration_S3EncryptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueSecurityConfiguration_EncryptionConfiguration_S3EncryptionRef {
        GlueSecurityConfiguration_EncryptionConfiguration_S3EncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueSecurityConfiguration_EncryptionConfiguration_S3EncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_encryption_mode` after provisioning.\n"]
    pub fn s3_encryption_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_encryption_mode", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueSecurityConfiguration_EncryptionConfigurationDynamic {
    cloudwatch_encryption: std
    ::option
    ::Option<terrars::DynamicBlock<GlueSecurityConfiguration_EncryptionConfiguration_CloudwatchEncryption>>,
    job_bookmarks_encryption: std
    ::option
    ::Option<terrars::DynamicBlock<GlueSecurityConfiguration_EncryptionConfiguration_JobBookmarksEncryption>>,
    s3_encryption: std
    ::option
    ::Option<terrars::DynamicBlock<GlueSecurityConfiguration_EncryptionConfiguration_S3Encryption>>,
}

#[derive(Serialize)]
pub struct GlueSecurityConfiguration_EncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_encryption: std
    ::option
    ::Option<std::vec::Vec<GlueSecurityConfiguration_EncryptionConfiguration_CloudwatchEncryption>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_bookmarks_encryption: std
    ::option
    ::Option<std::vec::Vec<GlueSecurityConfiguration_EncryptionConfiguration_JobBookmarksEncryption>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_encryption: std::option::Option<std::vec::Vec<GlueSecurityConfiguration_EncryptionConfiguration_S3Encryption>>,
    dynamic: GlueSecurityConfiguration_EncryptionConfigurationDynamic,
}

impl GlueSecurityConfiguration_EncryptionConfiguration {
    #[doc = "Set the field `cloudwatch_encryption`.\n"]
    pub fn set_cloudwatch_encryption(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GlueSecurityConfiguration_EncryptionConfiguration_CloudwatchEncryption>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_encryption = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `job_bookmarks_encryption`.\n"]
    pub fn set_job_bookmarks_encryption(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GlueSecurityConfiguration_EncryptionConfiguration_JobBookmarksEncryption>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.job_bookmarks_encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.job_bookmarks_encryption = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_encryption`.\n"]
    pub fn set_s3_encryption(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GlueSecurityConfiguration_EncryptionConfiguration_S3Encryption>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_encryption = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueSecurityConfiguration_EncryptionConfiguration {
    type O = terrars::BlockAssignable<GlueSecurityConfiguration_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueSecurityConfiguration_EncryptionConfiguration {}

impl BuildGlueSecurityConfiguration_EncryptionConfiguration {
    pub fn build(self) -> GlueSecurityConfiguration_EncryptionConfiguration {
        GlueSecurityConfiguration_EncryptionConfiguration {
            cloudwatch_encryption: core::default::Default::default(),
            job_bookmarks_encryption: core::default::Default::default(),
            s3_encryption: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueSecurityConfiguration_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueSecurityConfiguration_EncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueSecurityConfiguration_EncryptionConfigurationRef {
        GlueSecurityConfiguration_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueSecurityConfiguration_EncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_encryption` after provisioning.\n"]
    pub fn cloudwatch_encryption(
        &self,
    ) -> terrars::ListRef<GlueSecurityConfiguration_EncryptionConfiguration_CloudwatchEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `job_bookmarks_encryption` after provisioning.\n"]
    pub fn job_bookmarks_encryption(
        &self,
    ) -> terrars::ListRef<GlueSecurityConfiguration_EncryptionConfiguration_JobBookmarksEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.job_bookmarks_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_encryption` after provisioning.\n"]
    pub fn s3_encryption(&self) -> terrars::ListRef<GlueSecurityConfiguration_EncryptionConfiguration_S3EncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_encryption", self.base))
    }
}
