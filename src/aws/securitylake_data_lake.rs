#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SecuritylakeDataLakeData_ {
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
    meta_store_manager_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<SecuritylakeDataLake_Configuration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SecuritylakeDataLake_Timeouts>,
    dynamic: SecuritylakeDataLakeDynamic,
}

struct SecuritylakeDataLake_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SecuritylakeDataLakeData_>,
}

#[derive(Clone)]
pub struct SecuritylakeDataLake(std::rc::Rc<SecuritylakeDataLake_>);

impl SecuritylakeDataLake {
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

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecuritylakeDataLake_Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SecuritylakeDataLake_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `meta_store_manager_role_arn` after provisioning.\n"]
    pub fn meta_store_manager_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.meta_store_manager_role_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_arn` after provisioning.\n"]
    pub fn s3_bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<SecuritylakeDataLake_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecuritylakeDataLake_TimeoutsRef {
        SecuritylakeDataLake_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for SecuritylakeDataLake {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SecuritylakeDataLake { }

impl terrars::ToListMappable for SecuritylakeDataLake {
    type O = terrars::ListRef<SecuritylakeDataLakeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SecuritylakeDataLake_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_securitylake_data_lake".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecuritylakeDataLake {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub meta_store_manager_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildSecuritylakeDataLake {
    pub fn build(self, stack: &mut terrars::Stack) -> SecuritylakeDataLake {
        let out = SecuritylakeDataLake(std::rc::Rc::new(SecuritylakeDataLake_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SecuritylakeDataLakeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                meta_store_manager_role_arn: self.meta_store_manager_role_arn,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecuritylakeDataLakeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeDataLakeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SecuritylakeDataLakeRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `meta_store_manager_role_arn` after provisioning.\n"]
    pub fn meta_store_manager_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.meta_store_manager_role_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_arn` after provisioning.\n"]
    pub fn s3_bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<SecuritylakeDataLake_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecuritylakeDataLake_TimeoutsRef {
        SecuritylakeDataLake_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SecuritylakeDataLakeDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<SecuritylakeDataLake_Configuration>>,
}

#[derive(Serialize)]
pub struct SecuritylakeDataLake_Configuration_EncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecuritylakeDataLake_Configuration_EncryptionConfiguration {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecuritylakeDataLake_Configuration_EncryptionConfiguration {
    type O = terrars::BlockAssignable<SecuritylakeDataLake_Configuration_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeDataLake_Configuration_EncryptionConfiguration {}

impl BuildSecuritylakeDataLake_Configuration_EncryptionConfiguration {
    pub fn build(self) -> SecuritylakeDataLake_Configuration_EncryptionConfiguration {
        SecuritylakeDataLake_Configuration_EncryptionConfiguration { kms_key_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeDataLake_Configuration_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeDataLake_Configuration_EncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecuritylakeDataLake_Configuration_EncryptionConfigurationRef {
        SecuritylakeDataLake_Configuration_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeDataLake_Configuration_EncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeDataLake_Configuration_LifecycleConfiguration_Expiration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days: std::option::Option<terrars::PrimField<f64>>,
}

impl SecuritylakeDataLake_Configuration_LifecycleConfiguration_Expiration {
    #[doc = "Set the field `days`.\n"]
    pub fn set_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.days = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecuritylakeDataLake_Configuration_LifecycleConfiguration_Expiration {
    type O = terrars::BlockAssignable<SecuritylakeDataLake_Configuration_LifecycleConfiguration_Expiration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeDataLake_Configuration_LifecycleConfiguration_Expiration {}

impl BuildSecuritylakeDataLake_Configuration_LifecycleConfiguration_Expiration {
    pub fn build(self) -> SecuritylakeDataLake_Configuration_LifecycleConfiguration_Expiration {
        SecuritylakeDataLake_Configuration_LifecycleConfiguration_Expiration {
            days: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeDataLake_Configuration_LifecycleConfiguration_ExpirationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeDataLake_Configuration_LifecycleConfiguration_ExpirationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecuritylakeDataLake_Configuration_LifecycleConfiguration_ExpirationRef {
        SecuritylakeDataLake_Configuration_LifecycleConfiguration_ExpirationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeDataLake_Configuration_LifecycleConfiguration_ExpirationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeDataLake_Configuration_LifecycleConfiguration_Transition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecuritylakeDataLake_Configuration_LifecycleConfiguration_Transition {
    #[doc = "Set the field `days`.\n"]
    pub fn set_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_class`.\n"]
    pub fn set_storage_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_class = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecuritylakeDataLake_Configuration_LifecycleConfiguration_Transition {
    type O = terrars::BlockAssignable<SecuritylakeDataLake_Configuration_LifecycleConfiguration_Transition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeDataLake_Configuration_LifecycleConfiguration_Transition {}

impl BuildSecuritylakeDataLake_Configuration_LifecycleConfiguration_Transition {
    pub fn build(self) -> SecuritylakeDataLake_Configuration_LifecycleConfiguration_Transition {
        SecuritylakeDataLake_Configuration_LifecycleConfiguration_Transition {
            days: core::default::Default::default(),
            storage_class: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeDataLake_Configuration_LifecycleConfiguration_TransitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeDataLake_Configuration_LifecycleConfiguration_TransitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecuritylakeDataLake_Configuration_LifecycleConfiguration_TransitionRef {
        SecuritylakeDataLake_Configuration_LifecycleConfiguration_TransitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeDataLake_Configuration_LifecycleConfiguration_TransitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `days` after provisioning.\n"]
    pub fn days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_class` after provisioning.\n"]
    pub fn storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_class", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecuritylakeDataLake_Configuration_LifecycleConfigurationDynamic {
    expiration: std
    ::option
    ::Option<terrars::DynamicBlock<SecuritylakeDataLake_Configuration_LifecycleConfiguration_Expiration>>,
    transition: std
    ::option
    ::Option<terrars::DynamicBlock<SecuritylakeDataLake_Configuration_LifecycleConfiguration_Transition>>,
}

#[derive(Serialize)]
pub struct SecuritylakeDataLake_Configuration_LifecycleConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration: std::option::Option<std::vec::Vec<SecuritylakeDataLake_Configuration_LifecycleConfiguration_Expiration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transition: std::option::Option<std::vec::Vec<SecuritylakeDataLake_Configuration_LifecycleConfiguration_Transition>>,
    dynamic: SecuritylakeDataLake_Configuration_LifecycleConfigurationDynamic,
}

impl SecuritylakeDataLake_Configuration_LifecycleConfiguration {
    #[doc = "Set the field `expiration`.\n"]
    pub fn set_expiration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SecuritylakeDataLake_Configuration_LifecycleConfiguration_Expiration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.expiration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.expiration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `transition`.\n"]
    pub fn set_transition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SecuritylakeDataLake_Configuration_LifecycleConfiguration_Transition>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.transition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.transition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecuritylakeDataLake_Configuration_LifecycleConfiguration {
    type O = terrars::BlockAssignable<SecuritylakeDataLake_Configuration_LifecycleConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeDataLake_Configuration_LifecycleConfiguration {}

impl BuildSecuritylakeDataLake_Configuration_LifecycleConfiguration {
    pub fn build(self) -> SecuritylakeDataLake_Configuration_LifecycleConfiguration {
        SecuritylakeDataLake_Configuration_LifecycleConfiguration {
            expiration: core::default::Default::default(),
            transition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeDataLake_Configuration_LifecycleConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeDataLake_Configuration_LifecycleConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecuritylakeDataLake_Configuration_LifecycleConfigurationRef {
        SecuritylakeDataLake_Configuration_LifecycleConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeDataLake_Configuration_LifecycleConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expiration` after provisioning.\n"]
    pub fn expiration(
        &self,
    ) -> terrars::ListRef<SecuritylakeDataLake_Configuration_LifecycleConfiguration_ExpirationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expiration", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeDataLake_Configuration_ReplicationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecuritylakeDataLake_Configuration_ReplicationConfiguration {
    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.regions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecuritylakeDataLake_Configuration_ReplicationConfiguration {
    type O = terrars::BlockAssignable<SecuritylakeDataLake_Configuration_ReplicationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeDataLake_Configuration_ReplicationConfiguration {}

impl BuildSecuritylakeDataLake_Configuration_ReplicationConfiguration {
    pub fn build(self) -> SecuritylakeDataLake_Configuration_ReplicationConfiguration {
        SecuritylakeDataLake_Configuration_ReplicationConfiguration {
            regions: core::default::Default::default(),
            role_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeDataLake_Configuration_ReplicationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeDataLake_Configuration_ReplicationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecuritylakeDataLake_Configuration_ReplicationConfigurationRef {
        SecuritylakeDataLake_Configuration_ReplicationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeDataLake_Configuration_ReplicationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecuritylakeDataLake_ConfigurationDynamic {
    lifecycle_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<SecuritylakeDataLake_Configuration_LifecycleConfiguration>>,
    replication_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<SecuritylakeDataLake_Configuration_ReplicationConfiguration>>,
}

#[derive(Serialize)]
pub struct SecuritylakeDataLake_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std
    ::option
    ::Option<terrars::ListField<SecuritylakeDataLake_Configuration_EncryptionConfiguration>>,
    region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_configuration: std
    ::option
    ::Option<std::vec::Vec<SecuritylakeDataLake_Configuration_LifecycleConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_configuration: std
    ::option
    ::Option<std::vec::Vec<SecuritylakeDataLake_Configuration_ReplicationConfiguration>>,
    dynamic: SecuritylakeDataLake_ConfigurationDynamic,
}

impl SecuritylakeDataLake_Configuration {
    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        mut self,
        v: impl std::convert::Into<terrars::ListField<SecuritylakeDataLake_Configuration_EncryptionConfiguration>>,
    ) -> Self {
        self.encryption_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_configuration`.\n"]
    pub fn set_lifecycle_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecuritylakeDataLake_Configuration_LifecycleConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lifecycle_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lifecycle_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `replication_configuration`.\n"]
    pub fn set_replication_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecuritylakeDataLake_Configuration_ReplicationConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.replication_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.replication_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecuritylakeDataLake_Configuration {
    type O = terrars::BlockAssignable<SecuritylakeDataLake_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeDataLake_Configuration {
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildSecuritylakeDataLake_Configuration {
    pub fn build(self) -> SecuritylakeDataLake_Configuration {
        SecuritylakeDataLake_Configuration {
            encryption_configuration: core::default::Default::default(),
            region: self.region,
            lifecycle_configuration: core::default::Default::default(),
            replication_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeDataLake_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeDataLake_ConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecuritylakeDataLake_ConfigurationRef {
        SecuritylakeDataLake_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeDataLake_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> terrars::ListRef<SecuritylakeDataLake_Configuration_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_configuration` after provisioning.\n"]
    pub fn lifecycle_configuration(
        &self,
    ) -> terrars::ListRef<SecuritylakeDataLake_Configuration_LifecycleConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `replication_configuration` after provisioning.\n"]
    pub fn replication_configuration(
        &self,
    ) -> terrars::ListRef<SecuritylakeDataLake_Configuration_ReplicationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeDataLake_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecuritylakeDataLake_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecuritylakeDataLake_Timeouts {
    type O = terrars::BlockAssignable<SecuritylakeDataLake_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeDataLake_Timeouts {}

impl BuildSecuritylakeDataLake_Timeouts {
    pub fn build(self) -> SecuritylakeDataLake_Timeouts {
        SecuritylakeDataLake_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeDataLake_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeDataLake_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecuritylakeDataLake_TimeoutsRef {
        SecuritylakeDataLake_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeDataLake_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
