use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudwatchEventConnectionData {
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
    authorization_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_parameters: std::option::Option<std::vec::Vec<CloudwatchEventConnectionAuthParametersEl>>,
    dynamic: CloudwatchEventConnectionDynamic,
}

struct CloudwatchEventConnection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudwatchEventConnectionData>,
}

#[derive(Clone)]
pub struct CloudwatchEventConnection(std::rc::Rc<CloudwatchEventConnection_>);

impl CloudwatchEventConnection {
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

    #[doc = "Set the field `auth_parameters`.\n"]
    pub fn set_auth_parameters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventConnectionAuthParametersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auth_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auth_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorization_type` after provisioning.\n"]
    pub fn authorization_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_parameters` after provisioning.\n"]
    pub fn auth_parameters(&self) -> terrars::ListRef<CloudwatchEventConnectionAuthParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_parameters", self.extract_ref()))
    }
}

impl terrars::Referable for CloudwatchEventConnection {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudwatchEventConnection { }

impl terrars::ToListMappable for CloudwatchEventConnection {
    type O = terrars::ListRef<CloudwatchEventConnectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudwatchEventConnection_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudwatch_event_connection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudwatchEventConnection {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub authorization_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventConnection {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudwatchEventConnection {
        let out = CloudwatchEventConnection(std::rc::Rc::new(CloudwatchEventConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudwatchEventConnectionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                authorization_type: self.authorization_type,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                auth_parameters: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudwatchEventConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudwatchEventConnectionRef {
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

    #[doc = "Get a reference to the value of field `authorization_type` after provisioning.\n"]
    pub fn authorization_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_parameters` after provisioning.\n"]
    pub fn auth_parameters(&self) -> terrars::ListRef<CloudwatchEventConnectionAuthParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_parameters", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElApiKeyEl {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl CloudwatchEventConnectionAuthParametersElApiKeyEl { }

impl terrars::ToListMappable for CloudwatchEventConnectionAuthParametersElApiKeyEl {
    type O = terrars::BlockAssignable<CloudwatchEventConnectionAuthParametersElApiKeyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnectionAuthParametersElApiKeyEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventConnectionAuthParametersElApiKeyEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersElApiKeyEl {
        CloudwatchEventConnectionAuthParametersElApiKeyEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct CloudwatchEventConnectionAuthParametersElApiKeyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnectionAuthParametersElApiKeyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnectionAuthParametersElApiKeyElRef {
        CloudwatchEventConnectionAuthParametersElApiKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnectionAuthParametersElApiKeyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElBasicEl {
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl CloudwatchEventConnectionAuthParametersElBasicEl { }

impl terrars::ToListMappable for CloudwatchEventConnectionAuthParametersElBasicEl {
    type O = terrars::BlockAssignable<CloudwatchEventConnectionAuthParametersElBasicEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnectionAuthParametersElBasicEl {
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventConnectionAuthParametersElBasicEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersElBasicEl {
        CloudwatchEventConnectionAuthParametersElBasicEl {
            password: self.password,
            username: self.username,
        }
    }
}

pub struct CloudwatchEventConnectionAuthParametersElBasicElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnectionAuthParametersElBasicElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnectionAuthParametersElBasicElRef {
        CloudwatchEventConnectionAuthParametersElBasicElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnectionAuthParametersElBasicElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_value_secret: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl {
    #[doc = "Set the field `is_value_secret`.\n"]
    pub fn set_is_value_secret(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_value_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl {
    type O = terrars::BlockAssignable<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl {}

impl BuildCloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl {
        CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl {
            is_value_secret: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyElRef {
        CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_value_secret` after provisioning.\n"]
    pub fn is_value_secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_value_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_value_secret: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl {
    #[doc = "Set the field `is_value_secret`.\n"]
    pub fn set_is_value_secret(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_value_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl {
    type O = terrars::BlockAssignable<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl {}

impl BuildCloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl {
        CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl {
            is_value_secret: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderElRef {
        CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_value_secret` after provisioning.\n"]
    pub fn is_value_secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_value_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_value_secret: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl {
    #[doc = "Set the field `is_value_secret`.\n"]
    pub fn set_is_value_secret(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_value_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl {
    type O =
        terrars::BlockAssignable<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl {}

impl BuildCloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl {
        CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl {
            is_value_secret: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringElRef {
        CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_value_secret` after provisioning.\n"]
    pub fn is_value_secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_value_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElDynamic {
    body: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl>>,
    header: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl>>,
    query_string: std
    ::option
    ::Option<
        terrars::DynamicBlock<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl>,
    >,
}

#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    body: std::option::Option<std::vec::Vec<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std
    ::option
    ::Option<std::vec::Vec<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string: std
    ::option
    ::Option<std::vec::Vec<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl>>,
    dynamic: CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElDynamic,
}

impl CloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl {
    #[doc = "Set the field `body`.\n"]
    pub fn set_body(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.body = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.body = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_string`.\n"]
    pub fn set_query_string(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_string = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_string = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl {
    type O = terrars::BlockAssignable<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl {}

impl BuildCloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl {
        CloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl {
            body: core::default::Default::default(),
            header: core::default::Default::default(),
            query_string: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElRef {
        CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> terrars::ListRef<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElBodyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.body", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(
        &self,
    ) -> terrars::ListRef<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(
        &self,
    ) -> terrars::ListRef<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElQueryStringElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_string", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElOauthElClientParametersEl {
    client_id: terrars::PrimField<std::string::String>,
    client_secret: terrars::PrimField<std::string::String>,
}

impl CloudwatchEventConnectionAuthParametersElOauthElClientParametersEl { }

impl terrars::ToListMappable for CloudwatchEventConnectionAuthParametersElOauthElClientParametersEl {
    type O = terrars::BlockAssignable<CloudwatchEventConnectionAuthParametersElOauthElClientParametersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnectionAuthParametersElOauthElClientParametersEl {
    #[doc = ""]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub client_secret: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventConnectionAuthParametersElOauthElClientParametersEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersElOauthElClientParametersEl {
        CloudwatchEventConnectionAuthParametersElOauthElClientParametersEl {
            client_id: self.client_id,
            client_secret: self.client_secret,
        }
    }
}

pub struct CloudwatchEventConnectionAuthParametersElOauthElClientParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnectionAuthParametersElOauthElClientParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnectionAuthParametersElOauthElClientParametersElRef {
        CloudwatchEventConnectionAuthParametersElOauthElClientParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnectionAuthParametersElOauthElClientParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `client_id` after provisioning.\n"]
    pub fn client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_id", self.base))
    }

    #[doc = "Get a reference to the value of field `client_secret` after provisioning.\n"]
    pub fn client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_secret", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_value_secret: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl {
    #[doc = "Set the field `is_value_secret`.\n"]
    pub fn set_is_value_secret(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_value_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl {
    type O = terrars::BlockAssignable<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl {}

impl BuildCloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl {
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl {
            is_value_secret: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyElRef {
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_value_secret` after provisioning.\n"]
    pub fn is_value_secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_value_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_value_secret: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl {
    #[doc = "Set the field `is_value_secret`.\n"]
    pub fn set_is_value_secret(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_value_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl {
    type O = terrars::BlockAssignable<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl {}

impl BuildCloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl {
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl {
            is_value_secret: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderElRef {
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_value_secret` after provisioning.\n"]
    pub fn is_value_secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_value_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_value_secret: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl {
    #[doc = "Set the field `is_value_secret`.\n"]
    pub fn set_is_value_secret(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_value_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl {
    type O =
        terrars::BlockAssignable<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl {}

impl BuildCloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl {
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl {
            is_value_secret: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringElRef {
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `is_value_secret` after provisioning.\n"]
    pub fn is_value_secret(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_value_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElDynamic {
    body: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl>>,
    header: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl>>,
    query_string: std
    ::option
    ::Option<
        terrars::DynamicBlock<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl>,
    >,
}

#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    body: std::option::Option<std::vec::Vec<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std
    ::option
    ::Option<std::vec::Vec<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string: std
    ::option
    ::Option<std::vec::Vec<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl>>,
    dynamic: CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElDynamic,
}

impl CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl {
    #[doc = "Set the field `body`.\n"]
    pub fn set_body(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.body = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.body = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_string`.\n"]
    pub fn set_query_string(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_string = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_string = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl {
    type O = terrars::BlockAssignable<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl {}

impl BuildCloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl {
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl {
            body: core::default::Default::default(),
            header: core::default::Default::default(),
            query_string: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElRef {
        CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(
        &self,
    ) -> terrars::ListRef<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElBodyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.body", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(
        &self,
    ) -> terrars::ListRef<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(
        &self,
    ) -> terrars::ListRef<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElQueryStringElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_string", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventConnectionAuthParametersElOauthElDynamic {
    client_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnectionAuthParametersElOauthElClientParametersEl>>,
    oauth_http_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersElOauthEl {
    authorization_endpoint: terrars::PrimField<std::string::String>,
    http_method: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_parameters: std
    ::option
    ::Option<std::vec::Vec<CloudwatchEventConnectionAuthParametersElOauthElClientParametersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_http_parameters: std
    ::option
    ::Option<std::vec::Vec<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl>>,
    dynamic: CloudwatchEventConnectionAuthParametersElOauthElDynamic,
}

impl CloudwatchEventConnectionAuthParametersElOauthEl {
    #[doc = "Set the field `client_parameters`.\n"]
    pub fn set_client_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudwatchEventConnectionAuthParametersElOauthElClientParametersEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth_http_parameters`.\n"]
    pub fn set_oauth_http_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth_http_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth_http_parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventConnectionAuthParametersElOauthEl {
    type O = terrars::BlockAssignable<CloudwatchEventConnectionAuthParametersElOauthEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnectionAuthParametersElOauthEl {
    #[doc = ""]
    pub authorization_endpoint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub http_method: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventConnectionAuthParametersElOauthEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersElOauthEl {
        CloudwatchEventConnectionAuthParametersElOauthEl {
            authorization_endpoint: self.authorization_endpoint,
            http_method: self.http_method,
            client_parameters: core::default::Default::default(),
            oauth_http_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventConnectionAuthParametersElOauthElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnectionAuthParametersElOauthElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnectionAuthParametersElOauthElRef {
        CloudwatchEventConnectionAuthParametersElOauthElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnectionAuthParametersElOauthElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_endpoint` after provisioning.\n"]
    pub fn authorization_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `http_method` after provisioning.\n"]
    pub fn http_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_method", self.base))
    }

    #[doc = "Get a reference to the value of field `client_parameters` after provisioning.\n"]
    pub fn client_parameters(
        &self,
    ) -> terrars::ListRef<CloudwatchEventConnectionAuthParametersElOauthElClientParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_http_parameters` after provisioning.\n"]
    pub fn oauth_http_parameters(
        &self,
    ) -> terrars::ListRef<CloudwatchEventConnectionAuthParametersElOauthElOauthHttpParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_http_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventConnectionAuthParametersElDynamic {
    api_key: std::option::Option<terrars::DynamicBlock<CloudwatchEventConnectionAuthParametersElApiKeyEl>>,
    basic: std::option::Option<terrars::DynamicBlock<CloudwatchEventConnectionAuthParametersElBasicEl>>,
    invocation_http_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl>>,
    oauth: std::option::Option<terrars::DynamicBlock<CloudwatchEventConnectionAuthParametersElOauthEl>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventConnectionAuthParametersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<std::vec::Vec<CloudwatchEventConnectionAuthParametersElApiKeyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic: std::option::Option<std::vec::Vec<CloudwatchEventConnectionAuthParametersElBasicEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invocation_http_parameters: std
    ::option
    ::Option<std::vec::Vec<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth: std::option::Option<std::vec::Vec<CloudwatchEventConnectionAuthParametersElOauthEl>>,
    dynamic: CloudwatchEventConnectionAuthParametersElDynamic,
}

impl CloudwatchEventConnectionAuthParametersEl {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventConnectionAuthParametersElApiKeyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.api_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.api_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `basic`.\n"]
    pub fn set_basic(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventConnectionAuthParametersElBasicEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.basic = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.basic = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `invocation_http_parameters`.\n"]
    pub fn set_invocation_http_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.invocation_http_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.invocation_http_parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth`.\n"]
    pub fn set_oauth(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventConnectionAuthParametersElOauthEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventConnectionAuthParametersEl {
    type O = terrars::BlockAssignable<CloudwatchEventConnectionAuthParametersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnectionAuthParametersEl {}

impl BuildCloudwatchEventConnectionAuthParametersEl {
    pub fn build(self) -> CloudwatchEventConnectionAuthParametersEl {
        CloudwatchEventConnectionAuthParametersEl {
            api_key: core::default::Default::default(),
            basic: core::default::Default::default(),
            invocation_http_parameters: core::default::Default::default(),
            oauth: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudwatchEventConnectionAuthParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnectionAuthParametersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventConnectionAuthParametersElRef {
        CloudwatchEventConnectionAuthParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnectionAuthParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::ListRef<CloudwatchEventConnectionAuthParametersElApiKeyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `basic` after provisioning.\n"]
    pub fn basic(&self) -> terrars::ListRef<CloudwatchEventConnectionAuthParametersElBasicElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic", self.base))
    }

    #[doc = "Get a reference to the value of field `invocation_http_parameters` after provisioning.\n"]
    pub fn invocation_http_parameters(
        &self,
    ) -> terrars::ListRef<CloudwatchEventConnectionAuthParametersElInvocationHttpParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.invocation_http_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth` after provisioning.\n"]
    pub fn oauth(&self) -> terrars::ListRef<CloudwatchEventConnectionAuthParametersElOauthElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventConnectionDynamic {
    auth_parameters: std::option::Option<terrars::DynamicBlock<CloudwatchEventConnectionAuthParametersEl>>,
}
