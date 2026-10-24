use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3BucketCorsConfigurationData {
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
    cors_rule: std::option::Option<std::vec::Vec<S3BucketCorsConfigurationCorsRuleEl>>,
    dynamic: S3BucketCorsConfigurationDynamic,
}

struct S3BucketCorsConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3BucketCorsConfigurationData>,
}

#[derive(Clone)]
pub struct S3BucketCorsConfiguration(std::rc::Rc<S3BucketCorsConfiguration_>);

impl S3BucketCorsConfiguration {
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

    #[doc = "Set the field `cors_rule`.\n"]
    pub fn set_cors_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3BucketCorsConfigurationCorsRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cors_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cors_rule = std::option::Option::Some(d);
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
}

impl terrars::Referable for S3BucketCorsConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3BucketCorsConfiguration { }

impl terrars::ToListMappable for S3BucketCorsConfiguration {
    type O = terrars::ListRef<S3BucketCorsConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3BucketCorsConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3_bucket_cors_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3BucketCorsConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildS3BucketCorsConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> S3BucketCorsConfiguration {
        let out = S3BucketCorsConfiguration(std::rc::Rc::new(S3BucketCorsConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3BucketCorsConfigurationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket: self.bucket,
                expected_bucket_owner: core::default::Default::default(),
                id: core::default::Default::default(),
                cors_rule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3BucketCorsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketCorsConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3BucketCorsConfigurationRef {
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
}

#[derive(Serialize)]
pub struct S3BucketCorsConfigurationCorsRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    allowed_methods: terrars::SetField<terrars::PrimField<std::string::String>>,
    allowed_origins: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expose_headers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_age_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl S3BucketCorsConfigurationCorsRuleEl {
    #[doc = "Set the field `allowed_headers`.\n"]
    pub fn set_allowed_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expose_headers`.\n"]
    pub fn set_expose_headers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expose_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_age_seconds`.\n"]
    pub fn set_max_age_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_age_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3BucketCorsConfigurationCorsRuleEl {
    type O = terrars::BlockAssignable<S3BucketCorsConfigurationCorsRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3BucketCorsConfigurationCorsRuleEl {
    #[doc = ""]
    pub allowed_methods: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub allowed_origins: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildS3BucketCorsConfigurationCorsRuleEl {
    pub fn build(self) -> S3BucketCorsConfigurationCorsRuleEl {
        S3BucketCorsConfigurationCorsRuleEl {
            allowed_headers: core::default::Default::default(),
            allowed_methods: self.allowed_methods,
            allowed_origins: self.allowed_origins,
            expose_headers: core::default::Default::default(),
            id: core::default::Default::default(),
            max_age_seconds: core::default::Default::default(),
        }
    }
}

pub struct S3BucketCorsConfigurationCorsRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3BucketCorsConfigurationCorsRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3BucketCorsConfigurationCorsRuleElRef {
        S3BucketCorsConfigurationCorsRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3BucketCorsConfigurationCorsRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_headers` after provisioning.\n"]
    pub fn allowed_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_methods` after provisioning.\n"]
    pub fn allowed_methods(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_methods", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_origins` after provisioning.\n"]
    pub fn allowed_origins(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_origins", self.base))
    }

    #[doc = "Get a reference to the value of field `expose_headers` after provisioning.\n"]
    pub fn expose_headers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `max_age_seconds` after provisioning.\n"]
    pub fn max_age_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age_seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct S3BucketCorsConfigurationDynamic {
    cors_rule: std::option::Option<terrars::DynamicBlock<S3BucketCorsConfigurationCorsRuleEl>>,
}
