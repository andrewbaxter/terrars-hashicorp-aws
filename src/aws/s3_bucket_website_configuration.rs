#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketWebsiteConfigurationData_ {
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
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expected_bucket_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_rules: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_document: std::option::Option<std::vec::Vec<S3BucketWebsiteConfiguration_ErrorDocument>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index_document: std::option::Option<std::vec::Vec<S3BucketWebsiteConfiguration_IndexDocument>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect_all_requests_to: std::option::Option<std::vec::Vec<S3BucketWebsiteConfiguration_RedirectAllRequestsTo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_rule: std::option::Option<std::vec::Vec<S3BucketWebsiteConfiguration_RoutingRule>>,
    dynamic: S3BucketWebsiteConfigurationDynamic,
}

struct S3BucketWebsiteConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3BucketWebsiteConfigurationData_>,
}

#[derive(Clone)]
pub struct S3BucketWebsiteConfiguration(std::rc::Rc<S3BucketWebsiteConfiguration_>);

impl S3BucketWebsiteConfiguration {
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

    #[doc = "Set the field `expected_bucket_owner`.\n"]
    pub fn set_expected_bucket_owner(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().expected_bucket_owner = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `routing_rules`.\n"]
    pub fn set_routing_rules(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().routing_rules = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_document`.\n"]
    pub fn set_error_document(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketWebsiteConfiguration_ErrorDocument>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().error_document = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.error_document = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `index_document`.\n"]
    pub fn set_index_document(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketWebsiteConfiguration_IndexDocument>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().index_document = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.index_document = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redirect_all_requests_to`.\n"]
    pub fn set_redirect_all_requests_to(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketWebsiteConfiguration_RedirectAllRequestsTo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redirect_all_requests_to = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redirect_all_requests_to = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `routing_rule`.\n"]
    pub fn set_routing_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketWebsiteConfiguration_RoutingRule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().routing_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.routing_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `routing_rules` after provisioning.\n"]
    pub fn routing_rules(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `website_domain` after provisioning.\n"]
    pub fn website_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.website_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `website_endpoint` after provisioning.\n"]
    pub fn website_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.website_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_document` after provisioning.\n"]
    pub fn error_document(&self) -> terrars::ListRef<S3BucketWebsiteConfiguration_ErrorDocumentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_document", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `index_document` after provisioning.\n"]
    pub fn index_document(&self) -> terrars::ListRef<S3BucketWebsiteConfiguration_IndexDocumentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.index_document", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redirect_all_requests_to` after provisioning.\n"]
    pub fn redirect_all_requests_to(&self) -> terrars::ListRef<S3BucketWebsiteConfiguration_RedirectAllRequestsToRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect_all_requests_to", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_rule` after provisioning.\n"]
    pub fn routing_rule(&self) -> terrars::ListRef<S3BucketWebsiteConfiguration_RoutingRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing_rule", self.extract_ref()))
    }
}

impl terrars::Referable for S3BucketWebsiteConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3BucketWebsiteConfiguration { }

impl terrars::ToListMappable for S3BucketWebsiteConfiguration {
    type O = terrars::ListRef<S3BucketWebsiteConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3BucketWebsiteConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3_bucket_website_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketWebsiteConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketWebsiteConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> S3BucketWebsiteConfiguration {
        let out = S3BucketWebsiteConfiguration(std::rc::Rc::new(S3BucketWebsiteConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3BucketWebsiteConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket: self.bucket,
                expected_bucket_owner: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                routing_rules: core::default::Default::default(),
                error_document: core::default::Default::default(),
                index_document: core::default::Default::default(),
                redirect_all_requests_to: core::default::Default::default(),
                routing_rule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketWebsiteConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketWebsiteConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3BucketWebsiteConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expected_bucket_owner` after provisioning.\n"]
    pub fn expected_bucket_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expected_bucket_owner", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `routing_rules` after provisioning.\n"]
    pub fn routing_rules(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `website_domain` after provisioning.\n"]
    pub fn website_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.website_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `website_endpoint` after provisioning.\n"]
    pub fn website_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.website_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `error_document` after provisioning.\n"]
    pub fn error_document(&self) -> terrars::ListRef<S3BucketWebsiteConfiguration_ErrorDocumentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.error_document", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `index_document` after provisioning.\n"]
    pub fn index_document(&self) -> terrars::ListRef<S3BucketWebsiteConfiguration_IndexDocumentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.index_document", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redirect_all_requests_to` after provisioning.\n"]
    pub fn redirect_all_requests_to(&self) -> terrars::ListRef<S3BucketWebsiteConfiguration_RedirectAllRequestsToRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect_all_requests_to", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_rule` after provisioning.\n"]
    pub fn routing_rule(&self) -> terrars::ListRef<S3BucketWebsiteConfiguration_RoutingRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.routing_rule", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct S3BucketWebsiteConfigurationDynamic {
    error_document: std::option::Option<terrars::DynamicBlock<S3BucketWebsiteConfiguration_ErrorDocument>>,
    index_document: std::option::Option<terrars::DynamicBlock<S3BucketWebsiteConfiguration_IndexDocument>>,
    redirect_all_requests_to: std
    ::option
    ::Option<terrars::DynamicBlock<S3BucketWebsiteConfiguration_RedirectAllRequestsTo>>,
    routing_rule: std::option::Option<terrars::DynamicBlock<S3BucketWebsiteConfiguration_RoutingRule>>,
}

#[derive(Serialize)]
pub struct S3BucketWebsiteConfiguration_ErrorDocument {
    key: terrars::PrimField<std::string::String>,
}

impl S3BucketWebsiteConfiguration_ErrorDocument { }

impl terrars::ToListMappable for S3BucketWebsiteConfiguration_ErrorDocument {
    type O = terrars::BlockAssignable<S3BucketWebsiteConfiguration_ErrorDocument>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketWebsiteConfiguration_ErrorDocument {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketWebsiteConfiguration_ErrorDocument {
    pub fn build(self) -> S3BucketWebsiteConfiguration_ErrorDocument {
        S3BucketWebsiteConfiguration_ErrorDocument { key: self.key }
    }
}

#[allow(dead_code)]
pub struct S3BucketWebsiteConfiguration_ErrorDocumentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketWebsiteConfiguration_ErrorDocumentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketWebsiteConfiguration_ErrorDocumentRef {
        S3BucketWebsiteConfiguration_ErrorDocumentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketWebsiteConfiguration_ErrorDocumentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketWebsiteConfiguration_IndexDocument {
    suffix: terrars::PrimField<std::string::String>,
}

impl S3BucketWebsiteConfiguration_IndexDocument { }

impl terrars::ToListMappable for S3BucketWebsiteConfiguration_IndexDocument {
    type O = terrars::BlockAssignable<S3BucketWebsiteConfiguration_IndexDocument>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketWebsiteConfiguration_IndexDocument {
    #[doc = ""]
    pub suffix: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketWebsiteConfiguration_IndexDocument {
    pub fn build(self) -> S3BucketWebsiteConfiguration_IndexDocument {
        S3BucketWebsiteConfiguration_IndexDocument { suffix: self.suffix }
    }
}

#[allow(dead_code)]
pub struct S3BucketWebsiteConfiguration_IndexDocumentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketWebsiteConfiguration_IndexDocumentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketWebsiteConfiguration_IndexDocumentRef {
        S3BucketWebsiteConfiguration_IndexDocumentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketWebsiteConfiguration_IndexDocumentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `suffix` after provisioning.\n"]
    pub fn suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketWebsiteConfiguration_RedirectAllRequestsTo {
    host_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3BucketWebsiteConfiguration_RedirectAllRequestsTo {
    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketWebsiteConfiguration_RedirectAllRequestsTo {
    type O = terrars::BlockAssignable<S3BucketWebsiteConfiguration_RedirectAllRequestsTo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketWebsiteConfiguration_RedirectAllRequestsTo {
    #[doc = ""]
    pub host_name: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketWebsiteConfiguration_RedirectAllRequestsTo {
    pub fn build(self) -> S3BucketWebsiteConfiguration_RedirectAllRequestsTo {
        S3BucketWebsiteConfiguration_RedirectAllRequestsTo {
            host_name: self.host_name,
            protocol: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketWebsiteConfiguration_RedirectAllRequestsToRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketWebsiteConfiguration_RedirectAllRequestsToRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketWebsiteConfiguration_RedirectAllRequestsToRef {
        S3BucketWebsiteConfiguration_RedirectAllRequestsToRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketWebsiteConfiguration_RedirectAllRequestsToRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host_name` after provisioning.\n"]
    pub fn host_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_name", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketWebsiteConfiguration_RoutingRule_Condition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_error_code_returned_equals: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_prefix_equals: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3BucketWebsiteConfiguration_RoutingRule_Condition {
    #[doc = "Set the field `http_error_code_returned_equals`.\n"]
    pub fn set_http_error_code_returned_equals(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.http_error_code_returned_equals = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_prefix_equals`.\n"]
    pub fn set_key_prefix_equals(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_prefix_equals = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketWebsiteConfiguration_RoutingRule_Condition {
    type O = terrars::BlockAssignable<S3BucketWebsiteConfiguration_RoutingRule_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketWebsiteConfiguration_RoutingRule_Condition {}

impl BuildS3BucketWebsiteConfiguration_RoutingRule_Condition {
    pub fn build(self) -> S3BucketWebsiteConfiguration_RoutingRule_Condition {
        S3BucketWebsiteConfiguration_RoutingRule_Condition {
            http_error_code_returned_equals: core::default::Default::default(),
            key_prefix_equals: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketWebsiteConfiguration_RoutingRule_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketWebsiteConfiguration_RoutingRule_ConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketWebsiteConfiguration_RoutingRule_ConditionRef {
        S3BucketWebsiteConfiguration_RoutingRule_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketWebsiteConfiguration_RoutingRule_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_error_code_returned_equals` after provisioning.\n"]
    pub fn http_error_code_returned_equals(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_error_code_returned_equals", self.base))
    }

    #[doc = "Get a reference to the value of field `key_prefix_equals` after provisioning.\n"]
    pub fn key_prefix_equals(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_prefix_equals", self.base))
    }
}

#[derive(Serialize)]
pub struct S3BucketWebsiteConfiguration_RoutingRule_Redirect {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_redirect_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace_key_prefix_with: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace_key_with: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3BucketWebsiteConfiguration_RoutingRule_Redirect {
    #[doc = "Set the field `host_name`.\n"]
    pub fn set_host_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_redirect_code`.\n"]
    pub fn set_http_redirect_code(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.http_redirect_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replace_key_prefix_with`.\n"]
    pub fn set_replace_key_prefix_with(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.replace_key_prefix_with = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replace_key_with`.\n"]
    pub fn set_replace_key_with(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.replace_key_with = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketWebsiteConfiguration_RoutingRule_Redirect {
    type O = terrars::BlockAssignable<S3BucketWebsiteConfiguration_RoutingRule_Redirect>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketWebsiteConfiguration_RoutingRule_Redirect {}

impl BuildS3BucketWebsiteConfiguration_RoutingRule_Redirect {
    pub fn build(self) -> S3BucketWebsiteConfiguration_RoutingRule_Redirect {
        S3BucketWebsiteConfiguration_RoutingRule_Redirect {
            host_name: core::default::Default::default(),
            http_redirect_code: core::default::Default::default(),
            protocol: core::default::Default::default(),
            replace_key_prefix_with: core::default::Default::default(),
            replace_key_with: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketWebsiteConfiguration_RoutingRule_RedirectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketWebsiteConfiguration_RoutingRule_RedirectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3BucketWebsiteConfiguration_RoutingRule_RedirectRef {
        S3BucketWebsiteConfiguration_RoutingRule_RedirectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketWebsiteConfiguration_RoutingRule_RedirectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host_name` after provisioning.\n"]
    pub fn host_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_name", self.base))
    }

    #[doc = "Get a reference to the value of field `http_redirect_code` after provisioning.\n"]
    pub fn http_redirect_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_redirect_code", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `replace_key_prefix_with` after provisioning.\n"]
    pub fn replace_key_prefix_with(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace_key_prefix_with", self.base))
    }

    #[doc = "Get a reference to the value of field `replace_key_with` after provisioning.\n"]
    pub fn replace_key_with(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replace_key_with", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketWebsiteConfiguration_RoutingRuleDynamic {
    condition: std::option::Option<terrars::DynamicBlock<S3BucketWebsiteConfiguration_RoutingRule_Condition>>,
    redirect: std::option::Option<terrars::DynamicBlock<S3BucketWebsiteConfiguration_RoutingRule_Redirect>>,
}

#[derive(Serialize)]
pub struct S3BucketWebsiteConfiguration_RoutingRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<S3BucketWebsiteConfiguration_RoutingRule_Condition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redirect: std::option::Option<std::vec::Vec<S3BucketWebsiteConfiguration_RoutingRule_Redirect>>,
    dynamic: S3BucketWebsiteConfiguration_RoutingRuleDynamic,
}

impl S3BucketWebsiteConfiguration_RoutingRule {
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketWebsiteConfiguration_RoutingRule_Condition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redirect`.\n"]
    pub fn set_redirect(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketWebsiteConfiguration_RoutingRule_Redirect>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redirect = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redirect = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for S3BucketWebsiteConfiguration_RoutingRule {
    type O = terrars::BlockAssignable<S3BucketWebsiteConfiguration_RoutingRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketWebsiteConfiguration_RoutingRule {}

impl BuildS3BucketWebsiteConfiguration_RoutingRule {
    pub fn build(self) -> S3BucketWebsiteConfiguration_RoutingRule {
        S3BucketWebsiteConfiguration_RoutingRule {
            condition: core::default::Default::default(),
            redirect: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3BucketWebsiteConfiguration_RoutingRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketWebsiteConfiguration_RoutingRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketWebsiteConfiguration_RoutingRuleRef {
        S3BucketWebsiteConfiguration_RoutingRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketWebsiteConfiguration_RoutingRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<S3BucketWebsiteConfiguration_RoutingRule_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect` after provisioning.\n"]
    pub fn redirect(&self) -> terrars::ListRef<S3BucketWebsiteConfiguration_RoutingRule_RedirectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redirect", self.base))
    }
}
