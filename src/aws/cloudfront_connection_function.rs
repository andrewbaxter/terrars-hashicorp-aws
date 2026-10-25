#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontConnectionFunctionData_ {
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
    connection_function_code: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_function_config: std::option::Option<std::vec::Vec<CloudfrontConnectionFunction_ConnectionFunctionConfig>>,
    dynamic: CloudfrontConnectionFunctionDynamic,
}

struct CloudfrontConnectionFunction_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudfrontConnectionFunctionData_>,
}

#[derive(Clone)]
pub struct CloudfrontConnectionFunction(std::rc::Rc<CloudfrontConnectionFunction_>);

impl CloudfrontConnectionFunction {
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

    #[doc = "Set the field `publish`.\n"]
    pub fn set_publish(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().publish = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `connection_function_config`.\n"]
    pub fn set_connection_function_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontConnectionFunction_ConnectionFunctionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connection_function_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connection_function_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `connection_function_arn` after provisioning.\n"]
    pub fn connection_function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_function_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_function_code` after provisioning.\n"]
    pub fn connection_function_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_function_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `live_stage_etag` after provisioning.\n"]
    pub fn live_stage_etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.live_stage_etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publish` after provisioning.\n"]
    pub fn publish(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publish", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `connection_function_config` after provisioning.\n"]
    pub fn connection_function_config(
        &self,
    ) -> terrars::ListRef<CloudfrontConnectionFunction_ConnectionFunctionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_function_config", self.extract_ref()))
    }
}

impl terrars::Referable for CloudfrontConnectionFunction {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudfrontConnectionFunction { }

impl terrars::ToListMappable for CloudfrontConnectionFunction {
    type O = terrars::ListRef<CloudfrontConnectionFunctionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudfrontConnectionFunction_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudfront_connection_function".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontConnectionFunction {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub connection_function_code: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontConnectionFunction {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudfrontConnectionFunction {
        let out = CloudfrontConnectionFunction(std::rc::Rc::new(CloudfrontConnectionFunction_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudfrontConnectionFunctionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                connection_function_code: self.connection_function_code,
                name: self.name,
                publish: core::default::Default::default(),
                tags: core::default::Default::default(),
                connection_function_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontConnectionFunctionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontConnectionFunctionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudfrontConnectionFunctionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connection_function_arn` after provisioning.\n"]
    pub fn connection_function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_function_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_function_code` after provisioning.\n"]
    pub fn connection_function_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_function_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `live_stage_etag` after provisioning.\n"]
    pub fn live_stage_etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.live_stage_etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `publish` after provisioning.\n"]
    pub fn publish(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.publish", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `connection_function_config` after provisioning.\n"]
    pub fn connection_function_config(
        &self,
    ) -> terrars::ListRef<CloudfrontConnectionFunction_ConnectionFunctionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_function_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontConnectionFunctionDynamic {
    connection_function_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontConnectionFunction_ConnectionFunctionConfig>>,
}

#[derive(Serialize)]
pub struct CloudfrontConnectionFunction_ConnectionFunctionConfig_KeyValueStoreAssociation {
    key_value_store_arn: terrars::PrimField<std::string::String>,
}

impl CloudfrontConnectionFunction_ConnectionFunctionConfig_KeyValueStoreAssociation { }

impl terrars::ToListMappable for CloudfrontConnectionFunction_ConnectionFunctionConfig_KeyValueStoreAssociation {
    type O =
        terrars::BlockAssignable<CloudfrontConnectionFunction_ConnectionFunctionConfig_KeyValueStoreAssociation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontConnectionFunction_ConnectionFunctionConfig_KeyValueStoreAssociation {
    #[doc = ""]
    pub key_value_store_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontConnectionFunction_ConnectionFunctionConfig_KeyValueStoreAssociation {
    pub fn build(self) -> CloudfrontConnectionFunction_ConnectionFunctionConfig_KeyValueStoreAssociation {
        CloudfrontConnectionFunction_ConnectionFunctionConfig_KeyValueStoreAssociation {
            key_value_store_arn: self.key_value_store_arn,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontConnectionFunction_ConnectionFunctionConfig_KeyValueStoreAssociationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontConnectionFunction_ConnectionFunctionConfig_KeyValueStoreAssociationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontConnectionFunction_ConnectionFunctionConfig_KeyValueStoreAssociationRef {
        CloudfrontConnectionFunction_ConnectionFunctionConfig_KeyValueStoreAssociationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontConnectionFunction_ConnectionFunctionConfig_KeyValueStoreAssociationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_value_store_arn` after provisioning.\n"]
    pub fn key_value_store_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_value_store_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontConnectionFunction_ConnectionFunctionConfigDynamic {
    key_value_store_association: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontConnectionFunction_ConnectionFunctionConfig_KeyValueStoreAssociation>>,
}

#[derive(Serialize)]
pub struct CloudfrontConnectionFunction_ConnectionFunctionConfig {
    comment: terrars::PrimField<std::string::String>,
    runtime: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_value_store_association: std
    ::option
    ::Option<std::vec::Vec<CloudfrontConnectionFunction_ConnectionFunctionConfig_KeyValueStoreAssociation>>,
    dynamic: CloudfrontConnectionFunction_ConnectionFunctionConfigDynamic,
}

impl CloudfrontConnectionFunction_ConnectionFunctionConfig {
    #[doc = "Set the field `key_value_store_association`.\n"]
    pub fn set_key_value_store_association(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudfrontConnectionFunction_ConnectionFunctionConfig_KeyValueStoreAssociation,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.key_value_store_association = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.key_value_store_association = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontConnectionFunction_ConnectionFunctionConfig {
    type O = terrars::BlockAssignable<CloudfrontConnectionFunction_ConnectionFunctionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontConnectionFunction_ConnectionFunctionConfig {
    #[doc = ""]
    pub comment: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub runtime: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontConnectionFunction_ConnectionFunctionConfig {
    pub fn build(self) -> CloudfrontConnectionFunction_ConnectionFunctionConfig {
        CloudfrontConnectionFunction_ConnectionFunctionConfig {
            comment: self.comment,
            runtime: self.runtime,
            key_value_store_association: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontConnectionFunction_ConnectionFunctionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontConnectionFunction_ConnectionFunctionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontConnectionFunction_ConnectionFunctionConfigRef {
        CloudfrontConnectionFunction_ConnectionFunctionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontConnectionFunction_ConnectionFunctionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.base))
    }

    #[doc = "Get a reference to the value of field `runtime` after provisioning.\n"]
    pub fn runtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.runtime", self.base))
    }

    #[doc = "Get a reference to the value of field `key_value_store_association` after provisioning.\n"]
    pub fn key_value_store_association(
        &self,
    ) -> terrars::ListRef<CloudfrontConnectionFunction_ConnectionFunctionConfig_KeyValueStoreAssociationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.key_value_store_association", self.base))
    }
}
