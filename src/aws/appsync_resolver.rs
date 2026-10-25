#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppsyncResolverData_ {
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
    api_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source: std::option::Option<terrars::PrimField<std::string::String>>,
    field: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kind: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_batch_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_template: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    caching_config: std::option::Option<std::vec::Vec<AppsyncResolver_CachingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_config: std::option::Option<std::vec::Vec<AppsyncResolver_PipelineConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime: std::option::Option<std::vec::Vec<AppsyncResolver_Runtime>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_config: std::option::Option<std::vec::Vec<AppsyncResolver_SyncConfig>>,
    dynamic: AppsyncResolverDynamic,
}

struct AppsyncResolver_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppsyncResolverData_>,
}

#[derive(Clone)]
pub struct AppsyncResolver(std::rc::Rc<AppsyncResolver_>);

impl AppsyncResolver {
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

    #[doc = "Set the field `code`.\n"]
    pub fn set_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `data_source`.\n"]
    pub fn set_data_source(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().data_source = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kind`.\n"]
    pub fn set_kind(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kind = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_batch_size`.\n"]
    pub fn set_max_batch_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_batch_size = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_template`.\n"]
    pub fn set_request_template(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().request_template = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_template`.\n"]
    pub fn set_response_template(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().response_template = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `caching_config`.\n"]
    pub fn set_caching_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncResolver_CachingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().caching_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.caching_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pipeline_config`.\n"]
    pub fn set_pipeline_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncResolver_PipelineConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pipeline_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pipeline_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `runtime`.\n"]
    pub fn set_runtime(self, v: impl std::convert::Into<terrars::BlockAssignable<AppsyncResolver_Runtime>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().runtime = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.runtime = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sync_config`.\n"]
    pub fn set_sync_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncResolver_SyncConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sync_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sync_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kind` after provisioning.\n"]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_batch_size` after provisioning.\n"]
    pub fn max_batch_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_batch_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_template` after provisioning.\n"]
    pub fn request_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `response_template` after provisioning.\n"]
    pub fn response_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `caching_config` after provisioning.\n"]
    pub fn caching_config(&self) -> terrars::ListRef<AppsyncResolver_CachingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.caching_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_config` after provisioning.\n"]
    pub fn pipeline_config(&self) -> terrars::ListRef<AppsyncResolver_PipelineConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pipeline_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> terrars::ListRef<AppsyncResolver_RuntimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sync_config` after provisioning.\n"]
    pub fn sync_config(&self) -> terrars::ListRef<AppsyncResolver_SyncConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sync_config", self.extract_ref()))
    }
}

impl terrars::Referable for AppsyncResolver {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppsyncResolver { }

impl terrars::ToListMappable for AppsyncResolver {
    type O = terrars::ListRef<AppsyncResolverRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppsyncResolver_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appsync_resolver".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppsyncResolver {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub api_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub field: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncResolver {
    pub fn build(self, stack: &mut terrars::Stack) -> AppsyncResolver {
        let out = AppsyncResolver(std::rc::Rc::new(AppsyncResolver_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppsyncResolverData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                api_id: self.api_id,
                code: core::default::Default::default(),
                data_source: core::default::Default::default(),
                field: self.field,
                id: core::default::Default::default(),
                kind: core::default::Default::default(),
                max_batch_size: core::default::Default::default(),
                region: core::default::Default::default(),
                request_template: core::default::Default::default(),
                response_template: core::default::Default::default(),
                type_: self.type_,
                caching_config: core::default::Default::default(),
                pipeline_config: core::default::Default::default(),
                runtime: core::default::Default::default(),
                sync_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppsyncResolverRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncResolverRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppsyncResolverRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source` after provisioning.\n"]
    pub fn data_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kind` after provisioning.\n"]
    pub fn kind(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kind", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_batch_size` after provisioning.\n"]
    pub fn max_batch_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_batch_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `request_template` after provisioning.\n"]
    pub fn request_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.request_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `response_template` after provisioning.\n"]
    pub fn response_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `caching_config` after provisioning.\n"]
    pub fn caching_config(&self) -> terrars::ListRef<AppsyncResolver_CachingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.caching_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_config` after provisioning.\n"]
    pub fn pipeline_config(&self) -> terrars::ListRef<AppsyncResolver_PipelineConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pipeline_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> terrars::ListRef<AppsyncResolver_RuntimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sync_config` after provisioning.\n"]
    pub fn sync_config(&self) -> terrars::ListRef<AppsyncResolver_SyncConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sync_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AppsyncResolverDynamic {
    caching_config: std::option::Option<terrars::DynamicBlock<AppsyncResolver_CachingConfig>>,
    pipeline_config: std::option::Option<terrars::DynamicBlock<AppsyncResolver_PipelineConfig>>,
    runtime: std::option::Option<terrars::DynamicBlock<AppsyncResolver_Runtime>>,
    sync_config: std::option::Option<terrars::DynamicBlock<AppsyncResolver_SyncConfig>>,
}

#[derive(Serialize)]
pub struct AppsyncResolver_CachingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    caching_keys: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ttl: std::option::Option<terrars::PrimField<f64>>,
}

impl AppsyncResolver_CachingConfig {
    #[doc = "Set the field `caching_keys`.\n"]
    pub fn set_caching_keys(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.caching_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ttl`.\n"]
    pub fn set_ttl(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ttl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppsyncResolver_CachingConfig {
    type O = terrars::BlockAssignable<AppsyncResolver_CachingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncResolver_CachingConfig {}

impl BuildAppsyncResolver_CachingConfig {
    pub fn build(self) -> AppsyncResolver_CachingConfig {
        AppsyncResolver_CachingConfig {
            caching_keys: core::default::Default::default(),
            ttl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncResolver_CachingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncResolver_CachingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncResolver_CachingConfigRef {
        AppsyncResolver_CachingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncResolver_CachingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `caching_keys` after provisioning.\n"]
    pub fn caching_keys(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.caching_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `ttl` after provisioning.\n"]
    pub fn ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ttl", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncResolver_PipelineConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    functions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl AppsyncResolver_PipelineConfig {
    #[doc = "Set the field `functions`.\n"]
    pub fn set_functions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.functions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppsyncResolver_PipelineConfig {
    type O = terrars::BlockAssignable<AppsyncResolver_PipelineConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncResolver_PipelineConfig {}

impl BuildAppsyncResolver_PipelineConfig {
    pub fn build(self) -> AppsyncResolver_PipelineConfig {
        AppsyncResolver_PipelineConfig { functions: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AppsyncResolver_PipelineConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncResolver_PipelineConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncResolver_PipelineConfigRef {
        AppsyncResolver_PipelineConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncResolver_PipelineConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `functions` after provisioning.\n"]
    pub fn functions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.functions", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncResolver_Runtime {
    name: terrars::PrimField<std::string::String>,
    runtime_version: terrars::PrimField<std::string::String>,
}

impl AppsyncResolver_Runtime { }

impl terrars::ToListMappable for AppsyncResolver_Runtime {
    type O = terrars::BlockAssignable<AppsyncResolver_Runtime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncResolver_Runtime {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub runtime_version: terrars::PrimField<std::string::String>,
}

impl BuildAppsyncResolver_Runtime {
    pub fn build(self) -> AppsyncResolver_Runtime {
        AppsyncResolver_Runtime {
            name: self.name,
            runtime_version: self.runtime_version,
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncResolver_RuntimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncResolver_RuntimeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncResolver_RuntimeRef {
        AppsyncResolver_RuntimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncResolver_RuntimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime_version` after provisioning.\n"]
    pub fn runtime_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime_version", self.base))
    }
}

#[derive(Serialize)]
pub struct AppsyncResolver_SyncConfig_LambdaConflictHandlerConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_conflict_handler_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppsyncResolver_SyncConfig_LambdaConflictHandlerConfig {
    #[doc = "Set the field `lambda_conflict_handler_arn`.\n"]
    pub fn set_lambda_conflict_handler_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.lambda_conflict_handler_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppsyncResolver_SyncConfig_LambdaConflictHandlerConfig {
    type O = terrars::BlockAssignable<AppsyncResolver_SyncConfig_LambdaConflictHandlerConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncResolver_SyncConfig_LambdaConflictHandlerConfig {}

impl BuildAppsyncResolver_SyncConfig_LambdaConflictHandlerConfig {
    pub fn build(self) -> AppsyncResolver_SyncConfig_LambdaConflictHandlerConfig {
        AppsyncResolver_SyncConfig_LambdaConflictHandlerConfig {
            lambda_conflict_handler_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncResolver_SyncConfig_LambdaConflictHandlerConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncResolver_SyncConfig_LambdaConflictHandlerConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppsyncResolver_SyncConfig_LambdaConflictHandlerConfigRef {
        AppsyncResolver_SyncConfig_LambdaConflictHandlerConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncResolver_SyncConfig_LambdaConflictHandlerConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_conflict_handler_arn` after provisioning.\n"]
    pub fn lambda_conflict_handler_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_conflict_handler_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppsyncResolver_SyncConfigDynamic {
    lambda_conflict_handler_config: std
    ::option
    ::Option<terrars::DynamicBlock<AppsyncResolver_SyncConfig_LambdaConflictHandlerConfig>>,
}

#[derive(Serialize)]
pub struct AppsyncResolver_SyncConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conflict_detection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conflict_handler: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_conflict_handler_config: std
    ::option
    ::Option<std::vec::Vec<AppsyncResolver_SyncConfig_LambdaConflictHandlerConfig>>,
    dynamic: AppsyncResolver_SyncConfigDynamic,
}

impl AppsyncResolver_SyncConfig {
    #[doc = "Set the field `conflict_detection`.\n"]
    pub fn set_conflict_detection(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.conflict_detection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conflict_handler`.\n"]
    pub fn set_conflict_handler(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.conflict_handler = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lambda_conflict_handler_config`.\n"]
    pub fn set_lambda_conflict_handler_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppsyncResolver_SyncConfig_LambdaConflictHandlerConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_conflict_handler_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_conflict_handler_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppsyncResolver_SyncConfig {
    type O = terrars::BlockAssignable<AppsyncResolver_SyncConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppsyncResolver_SyncConfig {}

impl BuildAppsyncResolver_SyncConfig {
    pub fn build(self) -> AppsyncResolver_SyncConfig {
        AppsyncResolver_SyncConfig {
            conflict_detection: core::default::Default::default(),
            conflict_handler: core::default::Default::default(),
            lambda_conflict_handler_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppsyncResolver_SyncConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppsyncResolver_SyncConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppsyncResolver_SyncConfigRef {
        AppsyncResolver_SyncConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppsyncResolver_SyncConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conflict_detection` after provisioning.\n"]
    pub fn conflict_detection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conflict_detection", self.base))
    }

    #[doc = "Get a reference to the value of field `conflict_handler` after provisioning.\n"]
    pub fn conflict_handler(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conflict_handler", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_conflict_handler_config` after provisioning.\n"]
    pub fn lambda_conflict_handler_config(
        &self,
    ) -> terrars::ListRef<AppsyncResolver_SyncConfig_LambdaConflictHandlerConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_conflict_handler_config", self.base))
    }
}
