#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SecuritylakeSubscriberData_ {
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
    access_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscriber_description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscriber_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<SecuritylakeSubscriber_Source>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscriber_identity: std::option::Option<std::vec::Vec<SecuritylakeSubscriber_SubscriberIdentity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SecuritylakeSubscriber_Timeouts>,
    dynamic: SecuritylakeSubscriberDynamic,
}

struct SecuritylakeSubscriber_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SecuritylakeSubscriberData_>,
}

#[derive(Clone)]
pub struct SecuritylakeSubscriber(std::rc::Rc<SecuritylakeSubscriber_>);

impl SecuritylakeSubscriber {
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

    #[doc = "Set the field `access_type`.\n"]
    pub fn set_access_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().access_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subscriber_description`.\n"]
    pub fn set_subscriber_description(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().subscriber_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subscriber_name`.\n"]
    pub fn set_subscriber_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().subscriber_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecuritylakeSubscriber_Source>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `subscriber_identity`.\n"]
    pub fn set_subscriber_identity(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecuritylakeSubscriber_SubscriberIdentity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subscriber_identity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subscriber_identity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SecuritylakeSubscriber_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `access_type` after provisioning.\n"]
    pub fn access_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `resource_share_arn` after provisioning.\n"]
    pub fn resource_share_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_share_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_share_name` after provisioning.\n"]
    pub fn resource_share_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_share_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_arn` after provisioning.\n"]
    pub fn s3_bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subscriber_description` after provisioning.\n"]
    pub fn subscriber_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriber_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subscriber_endpoint` after provisioning.\n"]
    pub fn subscriber_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriber_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subscriber_name` after provisioning.\n"]
    pub fn subscriber_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriber_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subscriber_status` after provisioning.\n"]
    pub fn subscriber_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriber_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subscriber_identity` after provisioning.\n"]
    pub fn subscriber_identity(&self) -> terrars::ListRef<SecuritylakeSubscriber_SubscriberIdentityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subscriber_identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecuritylakeSubscriber_TimeoutsRef {
        SecuritylakeSubscriber_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for SecuritylakeSubscriber {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SecuritylakeSubscriber { }

impl terrars::ToListMappable for SecuritylakeSubscriber {
    type O = terrars::ListRef<SecuritylakeSubscriberRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SecuritylakeSubscriber_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_securitylake_subscriber".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecuritylakeSubscriber {
    pub tf_id: std::string::String,
}

impl BuildSecuritylakeSubscriber {
    pub fn build(self, stack: &mut terrars::Stack) -> SecuritylakeSubscriber {
        let out = SecuritylakeSubscriber(std::rc::Rc::new(SecuritylakeSubscriber_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SecuritylakeSubscriberData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                access_type: core::default::Default::default(),
                region: core::default::Default::default(),
                subscriber_description: core::default::Default::default(),
                subscriber_name: core::default::Default::default(),
                tags: core::default::Default::default(),
                source: core::default::Default::default(),
                subscriber_identity: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecuritylakeSubscriberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeSubscriberRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SecuritylakeSubscriberRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_type` after provisioning.\n"]
    pub fn access_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `resource_share_arn` after provisioning.\n"]
    pub fn resource_share_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_share_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_share_name` after provisioning.\n"]
    pub fn resource_share_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_share_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_arn` after provisioning.\n"]
    pub fn s3_bucket_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subscriber_description` after provisioning.\n"]
    pub fn subscriber_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriber_description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subscriber_endpoint` after provisioning.\n"]
    pub fn subscriber_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriber_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subscriber_name` after provisioning.\n"]
    pub fn subscriber_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriber_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subscriber_status` after provisioning.\n"]
    pub fn subscriber_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriber_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subscriber_identity` after provisioning.\n"]
    pub fn subscriber_identity(&self) -> terrars::ListRef<SecuritylakeSubscriber_SubscriberIdentityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subscriber_identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SecuritylakeSubscriber_TimeoutsRef {
        SecuritylakeSubscriber_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SecuritylakeSubscriberDynamic {
    source: std::option::Option<terrars::DynamicBlock<SecuritylakeSubscriber_Source>>,
    subscriber_identity: std::option::Option<terrars::DynamicBlock<SecuritylakeSubscriber_SubscriberIdentity>>,
}

#[derive(Serialize)]
pub struct SecuritylakeSubscriber_Source_AwsLogSourceResource {
    source_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecuritylakeSubscriber_Source_AwsLogSourceResource {
    #[doc = "Set the field `source_version`.\n"]
    pub fn set_source_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecuritylakeSubscriber_Source_AwsLogSourceResource {
    type O = terrars::BlockAssignable<SecuritylakeSubscriber_Source_AwsLogSourceResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeSubscriber_Source_AwsLogSourceResource {
    #[doc = ""]
    pub source_name: terrars::PrimField<std::string::String>,
}

impl BuildSecuritylakeSubscriber_Source_AwsLogSourceResource {
    pub fn build(self) -> SecuritylakeSubscriber_Source_AwsLogSourceResource {
        SecuritylakeSubscriber_Source_AwsLogSourceResource {
            source_name: self.source_name,
            source_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeSubscriber_Source_AwsLogSourceResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeSubscriber_Source_AwsLogSourceResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecuritylakeSubscriber_Source_AwsLogSourceResourceRef {
        SecuritylakeSubscriber_Source_AwsLogSourceResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeSubscriber_Source_AwsLogSourceResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_name` after provisioning.\n"]
    pub fn source_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_name", self.base))
    }

    #[doc = "Get a reference to the value of field `source_version` after provisioning.\n"]
    pub fn source_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_version", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeSubscriber_Source_CustomLogSourceResource_Attributes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crawler_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    database_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    table_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecuritylakeSubscriber_Source_CustomLogSourceResource_Attributes {
    #[doc = "Set the field `crawler_arn`.\n"]
    pub fn set_crawler_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.crawler_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `database_arn`.\n"]
    pub fn set_database_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.database_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `table_arn`.\n"]
    pub fn set_table_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.table_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecuritylakeSubscriber_Source_CustomLogSourceResource_Attributes {
    type O = terrars::BlockAssignable<SecuritylakeSubscriber_Source_CustomLogSourceResource_Attributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeSubscriber_Source_CustomLogSourceResource_Attributes {}

impl BuildSecuritylakeSubscriber_Source_CustomLogSourceResource_Attributes {
    pub fn build(self) -> SecuritylakeSubscriber_Source_CustomLogSourceResource_Attributes {
        SecuritylakeSubscriber_Source_CustomLogSourceResource_Attributes {
            crawler_arn: core::default::Default::default(),
            database_arn: core::default::Default::default(),
            table_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeSubscriber_Source_CustomLogSourceResource_AttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeSubscriber_Source_CustomLogSourceResource_AttributesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecuritylakeSubscriber_Source_CustomLogSourceResource_AttributesRef {
        SecuritylakeSubscriber_Source_CustomLogSourceResource_AttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeSubscriber_Source_CustomLogSourceResource_AttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `crawler_arn` after provisioning.\n"]
    pub fn crawler_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crawler_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `database_arn` after provisioning.\n"]
    pub fn database_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `table_arn` after provisioning.\n"]
    pub fn table_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeSubscriber_Source_CustomLogSourceResource_Provider {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecuritylakeSubscriber_Source_CustomLogSourceResource_Provider {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecuritylakeSubscriber_Source_CustomLogSourceResource_Provider {
    type O = terrars::BlockAssignable<SecuritylakeSubscriber_Source_CustomLogSourceResource_Provider>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeSubscriber_Source_CustomLogSourceResource_Provider {}

impl BuildSecuritylakeSubscriber_Source_CustomLogSourceResource_Provider {
    pub fn build(self) -> SecuritylakeSubscriber_Source_CustomLogSourceResource_Provider {
        SecuritylakeSubscriber_Source_CustomLogSourceResource_Provider {
            location: core::default::Default::default(),
            role_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeSubscriber_Source_CustomLogSourceResource_ProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeSubscriber_Source_CustomLogSourceResource_ProviderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecuritylakeSubscriber_Source_CustomLogSourceResource_ProviderRef {
        SecuritylakeSubscriber_Source_CustomLogSourceResource_ProviderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeSubscriber_Source_CustomLogSourceResource_ProviderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeSubscriber_Source_CustomLogSourceResource {
    source_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecuritylakeSubscriber_Source_CustomLogSourceResource {
    #[doc = "Set the field `source_version`.\n"]
    pub fn set_source_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecuritylakeSubscriber_Source_CustomLogSourceResource {
    type O = terrars::BlockAssignable<SecuritylakeSubscriber_Source_CustomLogSourceResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeSubscriber_Source_CustomLogSourceResource {
    #[doc = ""]
    pub source_name: terrars::PrimField<std::string::String>,
}

impl BuildSecuritylakeSubscriber_Source_CustomLogSourceResource {
    pub fn build(self) -> SecuritylakeSubscriber_Source_CustomLogSourceResource {
        SecuritylakeSubscriber_Source_CustomLogSourceResource {
            source_name: self.source_name,
            source_version: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeSubscriber_Source_CustomLogSourceResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeSubscriber_Source_CustomLogSourceResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecuritylakeSubscriber_Source_CustomLogSourceResourceRef {
        SecuritylakeSubscriber_Source_CustomLogSourceResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeSubscriber_Source_CustomLogSourceResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> terrars::ListRef<SecuritylakeSubscriber_Source_CustomLogSourceResource_AttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attributes", self.base))
    }

    #[doc = "Get a reference to the value of field `provider` after provisioning.\n"]
    pub fn provider(&self) -> terrars::ListRef<SecuritylakeSubscriber_Source_CustomLogSourceResource_ProviderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.provider", self.base))
    }

    #[doc = "Get a reference to the value of field `source_name` after provisioning.\n"]
    pub fn source_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_name", self.base))
    }

    #[doc = "Get a reference to the value of field `source_version` after provisioning.\n"]
    pub fn source_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecuritylakeSubscriber_SourceDynamic {
    aws_log_source_resource: std
    ::option
    ::Option<terrars::DynamicBlock<SecuritylakeSubscriber_Source_AwsLogSourceResource>>,
    custom_log_source_resource: std
    ::option
    ::Option<terrars::DynamicBlock<SecuritylakeSubscriber_Source_CustomLogSourceResource>>,
}

#[derive(Serialize)]
pub struct SecuritylakeSubscriber_Source {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_log_source_resource: std::option::Option<std::vec::Vec<SecuritylakeSubscriber_Source_AwsLogSourceResource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_log_source_resource: std::option::Option<std::vec::Vec<SecuritylakeSubscriber_Source_CustomLogSourceResource>>,
    dynamic: SecuritylakeSubscriber_SourceDynamic,
}

impl SecuritylakeSubscriber_Source {
    #[doc = "Set the field `aws_log_source_resource`.\n"]
    pub fn set_aws_log_source_resource(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecuritylakeSubscriber_Source_AwsLogSourceResource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_log_source_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_log_source_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_log_source_resource`.\n"]
    pub fn set_custom_log_source_resource(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecuritylakeSubscriber_Source_CustomLogSourceResource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_log_source_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_log_source_resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecuritylakeSubscriber_Source {
    type O = terrars::BlockAssignable<SecuritylakeSubscriber_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeSubscriber_Source {}

impl BuildSecuritylakeSubscriber_Source {
    pub fn build(self) -> SecuritylakeSubscriber_Source {
        SecuritylakeSubscriber_Source {
            aws_log_source_resource: core::default::Default::default(),
            custom_log_source_resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeSubscriber_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeSubscriber_SourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecuritylakeSubscriber_SourceRef {
        SecuritylakeSubscriber_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeSubscriber_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_log_source_resource` after provisioning.\n"]
    pub fn aws_log_source_resource(&self) -> terrars::ListRef<SecuritylakeSubscriber_Source_AwsLogSourceResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_log_source_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_log_source_resource` after provisioning.\n"]
    pub fn custom_log_source_resource(
        &self,
    ) -> terrars::ListRef<SecuritylakeSubscriber_Source_CustomLogSourceResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_log_source_resource", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeSubscriber_SubscriberIdentity {
    external_id: terrars::PrimField<std::string::String>,
    principal: terrars::PrimField<std::string::String>,
}

impl SecuritylakeSubscriber_SubscriberIdentity { }

impl terrars::ToListMappable for SecuritylakeSubscriber_SubscriberIdentity {
    type O = terrars::BlockAssignable<SecuritylakeSubscriber_SubscriberIdentity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeSubscriber_SubscriberIdentity {
    #[doc = ""]
    pub external_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub principal: terrars::PrimField<std::string::String>,
}

impl BuildSecuritylakeSubscriber_SubscriberIdentity {
    pub fn build(self) -> SecuritylakeSubscriber_SubscriberIdentity {
        SecuritylakeSubscriber_SubscriberIdentity {
            external_id: self.external_id,
            principal: self.principal,
        }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeSubscriber_SubscriberIdentityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeSubscriber_SubscriberIdentityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecuritylakeSubscriber_SubscriberIdentityRef {
        SecuritylakeSubscriber_SubscriberIdentityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeSubscriber_SubscriberIdentityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.base))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeSubscriber_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecuritylakeSubscriber_Timeouts {
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

impl terrars::ToListMappable for SecuritylakeSubscriber_Timeouts {
    type O = terrars::BlockAssignable<SecuritylakeSubscriber_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeSubscriber_Timeouts {}

impl BuildSecuritylakeSubscriber_Timeouts {
    pub fn build(self) -> SecuritylakeSubscriber_Timeouts {
        SecuritylakeSubscriber_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeSubscriber_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeSubscriber_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecuritylakeSubscriber_TimeoutsRef {
        SecuritylakeSubscriber_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeSubscriber_TimeoutsRef {
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
