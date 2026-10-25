#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CleanroomsMembershipData_ {
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
    collaboration_id: terrars::PrimField<std::string::String>,
    query_log_status: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_result_configuration: std::option::Option<std::vec::Vec<CleanroomsMembership_DefaultResultConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_configuration: std::option::Option<std::vec::Vec<CleanroomsMembership_PaymentConfiguration>>,
    dynamic: CleanroomsMembershipDynamic,
}

struct CleanroomsMembership_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CleanroomsMembershipData_>,
}

#[derive(Clone)]
pub struct CleanroomsMembership(std::rc::Rc<CleanroomsMembership_>);

impl CleanroomsMembership {
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

    #[doc = "Set the field `default_result_configuration`.\n"]
    pub fn set_default_result_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CleanroomsMembership_DefaultResultConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_result_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_result_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `payment_configuration`.\n"]
    pub fn set_payment_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CleanroomsMembership_PaymentConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().payment_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.payment_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collaboration_arn` after provisioning.\n"]
    pub fn collaboration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collaboration_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collaboration_creator_account_id` after provisioning.\n"]
    pub fn collaboration_creator_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaboration_creator_account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `collaboration_creator_display_name` after provisioning.\n"]
    pub fn collaboration_creator_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaboration_creator_display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `collaboration_id` after provisioning.\n"]
    pub fn collaboration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collaboration_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collaboration_name` after provisioning.\n"]
    pub fn collaboration_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collaboration_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `member_abilities` after provisioning.\n"]
    pub fn member_abilities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.member_abilities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_log_status` after provisioning.\n"]
    pub fn query_log_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_log_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_result_configuration` after provisioning.\n"]
    pub fn default_result_configuration(&self) -> terrars::ListRef<CleanroomsMembership_DefaultResultConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_result_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `payment_configuration` after provisioning.\n"]
    pub fn payment_configuration(&self) -> terrars::ListRef<CleanroomsMembership_PaymentConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.payment_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for CleanroomsMembership {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CleanroomsMembership { }

impl terrars::ToListMappable for CleanroomsMembership {
    type O = terrars::ListRef<CleanroomsMembershipRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CleanroomsMembership_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cleanrooms_membership".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCleanroomsMembership {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub collaboration_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub query_log_status: terrars::PrimField<std::string::String>,
}

impl BuildCleanroomsMembership {
    pub fn build(self, stack: &mut terrars::Stack) -> CleanroomsMembership {
        let out = CleanroomsMembership(std::rc::Rc::new(CleanroomsMembership_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CleanroomsMembershipData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                collaboration_id: self.collaboration_id,
                query_log_status: self.query_log_status,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                default_result_configuration: core::default::Default::default(),
                payment_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CleanroomsMembershipRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CleanroomsMembershipRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CleanroomsMembershipRef {
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

    #[doc = "Get a reference to the value of field `collaboration_arn` after provisioning.\n"]
    pub fn collaboration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collaboration_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collaboration_creator_account_id` after provisioning.\n"]
    pub fn collaboration_creator_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaboration_creator_account_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `collaboration_creator_display_name` after provisioning.\n"]
    pub fn collaboration_creator_display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.collaboration_creator_display_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `collaboration_id` after provisioning.\n"]
    pub fn collaboration_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collaboration_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `collaboration_name` after provisioning.\n"]
    pub fn collaboration_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.collaboration_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `create_time` after provisioning.\n"]
    pub fn create_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `member_abilities` after provisioning.\n"]
    pub fn member_abilities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.member_abilities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `query_log_status` after provisioning.\n"]
    pub fn query_log_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query_log_status", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_time` after provisioning.\n"]
    pub fn update_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_result_configuration` after provisioning.\n"]
    pub fn default_result_configuration(&self) -> terrars::ListRef<CleanroomsMembership_DefaultResultConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_result_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `payment_configuration` after provisioning.\n"]
    pub fn payment_configuration(&self) -> terrars::ListRef<CleanroomsMembership_PaymentConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.payment_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CleanroomsMembershipDynamic {
    default_result_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CleanroomsMembership_DefaultResultConfiguration>>,
    payment_configuration: std::option::Option<terrars::DynamicBlock<CleanroomsMembership_PaymentConfiguration>>,
}

#[derive(Serialize)]
pub struct CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration_S3 {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    result_format: terrars::PrimField<std::string::String>,
}

impl CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration_S3 {
    #[doc = "Set the field `key_prefix`.\n"]
    pub fn set_key_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration_S3 {
    type O = terrars::BlockAssignable<CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCleanroomsMembership_DefaultResultConfiguration_OutputConfiguration_S3 {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub result_format: terrars::PrimField<std::string::String>,
}

impl BuildCleanroomsMembership_DefaultResultConfiguration_OutputConfiguration_S3 {
    pub fn build(self) -> CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration_S3 {
        CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration_S3 {
            bucket: self.bucket,
            key_prefix: core::default::Default::default(),
            result_format: self.result_format,
        }
    }
}

#[allow(dead_code)]
pub struct CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration_S3Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration_S3Ref {
        CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `key_prefix` after provisioning.\n"]
    pub fn key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `result_format` after provisioning.\n"]
    pub fn result_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.result_format", self.base))
    }
}

#[derive(Serialize, Default)]
struct CleanroomsMembership_DefaultResultConfiguration_OutputConfigurationDynamic {
    s3: std
    ::option
    ::Option<terrars::DynamicBlock<CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration_S3>>,
}

#[derive(Serialize)]
pub struct CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration_S3>>,
    dynamic: CleanroomsMembership_DefaultResultConfiguration_OutputConfigurationDynamic,
}

impl CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration_S3>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration {
    type O = terrars::BlockAssignable<CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCleanroomsMembership_DefaultResultConfiguration_OutputConfiguration {}

impl BuildCleanroomsMembership_DefaultResultConfiguration_OutputConfiguration {
    pub fn build(self) -> CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration {
        CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration {
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CleanroomsMembership_DefaultResultConfiguration_OutputConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CleanroomsMembership_DefaultResultConfiguration_OutputConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CleanroomsMembership_DefaultResultConfiguration_OutputConfigurationRef {
        CleanroomsMembership_DefaultResultConfiguration_OutputConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CleanroomsMembership_DefaultResultConfiguration_OutputConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct CleanroomsMembership_DefaultResultConfigurationDynamic {
    output_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration>>,
}

#[derive(Serialize)]
pub struct CleanroomsMembership_DefaultResultConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_configuration: std
    ::option
    ::Option<std::vec::Vec<CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration>>,
    dynamic: CleanroomsMembership_DefaultResultConfigurationDynamic,
}

impl CleanroomsMembership_DefaultResultConfiguration {
    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `output_configuration`.\n"]
    pub fn set_output_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CleanroomsMembership_DefaultResultConfiguration_OutputConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.output_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.output_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CleanroomsMembership_DefaultResultConfiguration {
    type O = terrars::BlockAssignable<CleanroomsMembership_DefaultResultConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCleanroomsMembership_DefaultResultConfiguration {}

impl BuildCleanroomsMembership_DefaultResultConfiguration {
    pub fn build(self) -> CleanroomsMembership_DefaultResultConfiguration {
        CleanroomsMembership_DefaultResultConfiguration {
            role_arn: core::default::Default::default(),
            output_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CleanroomsMembership_DefaultResultConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CleanroomsMembership_DefaultResultConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CleanroomsMembership_DefaultResultConfigurationRef {
        CleanroomsMembership_DefaultResultConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CleanroomsMembership_DefaultResultConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `output_configuration` after provisioning.\n"]
    pub fn output_configuration(
        &self,
    ) -> terrars::ListRef<CleanroomsMembership_DefaultResultConfiguration_OutputConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct CleanroomsMembership_PaymentConfiguration_QueryCompute {
    is_responsible: terrars::PrimField<bool>,
}

impl CleanroomsMembership_PaymentConfiguration_QueryCompute { }

impl terrars::ToListMappable for CleanroomsMembership_PaymentConfiguration_QueryCompute {
    type O = terrars::BlockAssignable<CleanroomsMembership_PaymentConfiguration_QueryCompute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCleanroomsMembership_PaymentConfiguration_QueryCompute {
    #[doc = ""]
    pub is_responsible: terrars::PrimField<bool>,
}

impl BuildCleanroomsMembership_PaymentConfiguration_QueryCompute {
    pub fn build(self) -> CleanroomsMembership_PaymentConfiguration_QueryCompute {
        CleanroomsMembership_PaymentConfiguration_QueryCompute { is_responsible: self.is_responsible }
    }
}

#[allow(dead_code)]
pub struct CleanroomsMembership_PaymentConfiguration_QueryComputeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CleanroomsMembership_PaymentConfiguration_QueryComputeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CleanroomsMembership_PaymentConfiguration_QueryComputeRef {
        CleanroomsMembership_PaymentConfiguration_QueryComputeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CleanroomsMembership_PaymentConfiguration_QueryComputeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_responsible` after provisioning.\n"]
    pub fn is_responsible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_responsible", self.base))
    }
}

#[derive(Serialize, Default)]
struct CleanroomsMembership_PaymentConfigurationDynamic {
    query_compute: std
    ::option
    ::Option<terrars::DynamicBlock<CleanroomsMembership_PaymentConfiguration_QueryCompute>>,
}

#[derive(Serialize)]
pub struct CleanroomsMembership_PaymentConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_compute: std::option::Option<std::vec::Vec<CleanroomsMembership_PaymentConfiguration_QueryCompute>>,
    dynamic: CleanroomsMembership_PaymentConfigurationDynamic,
}

impl CleanroomsMembership_PaymentConfiguration {
    #[doc = "Set the field `query_compute`.\n"]
    pub fn set_query_compute(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CleanroomsMembership_PaymentConfiguration_QueryCompute>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_compute = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_compute = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CleanroomsMembership_PaymentConfiguration {
    type O = terrars::BlockAssignable<CleanroomsMembership_PaymentConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCleanroomsMembership_PaymentConfiguration {}

impl BuildCleanroomsMembership_PaymentConfiguration {
    pub fn build(self) -> CleanroomsMembership_PaymentConfiguration {
        CleanroomsMembership_PaymentConfiguration {
            query_compute: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CleanroomsMembership_PaymentConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CleanroomsMembership_PaymentConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CleanroomsMembership_PaymentConfigurationRef {
        CleanroomsMembership_PaymentConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CleanroomsMembership_PaymentConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `query_compute` after provisioning.\n"]
    pub fn query_compute(&self) -> terrars::ListRef<CleanroomsMembership_PaymentConfiguration_QueryComputeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_compute", self.base))
    }
}
