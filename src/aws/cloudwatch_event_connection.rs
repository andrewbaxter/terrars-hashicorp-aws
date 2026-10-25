#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudwatchEventConnectionData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auth_parameters: std::option::Option<std::vec::Vec<CloudwatchEventConnection_AuthParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invocation_connectivity_parameters: std
    ::option
    ::Option<std::vec::Vec<CloudwatchEventConnection_InvocationConnectivityParameters>>,
    dynamic: CloudwatchEventConnectionDynamic,
}

struct CloudwatchEventConnection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudwatchEventConnectionData_>,
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

    #[doc = "Set the field `kms_key_identifier`.\n"]
    pub fn set_kms_key_identifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auth_parameters`.\n"]
    pub fn set_auth_parameters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventConnection_AuthParameters>>,
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

    #[doc = "Set the field `invocation_connectivity_parameters`.\n"]
    pub fn set_invocation_connectivity_parameters(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchEventConnection_InvocationConnectivityParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().invocation_connectivity_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.invocation_connectivity_parameters = std::option::Option::Some(d);
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

    #[doc = "Get a reference to the value of field `kms_key_identifier` after provisioning.\n"]
    pub fn kms_key_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_identifier", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_parameters` after provisioning.\n"]
    pub fn auth_parameters(&self) -> terrars::ListRef<CloudwatchEventConnection_AuthParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `invocation_connectivity_parameters` after provisioning.\n"]
    pub fn invocation_connectivity_parameters(
        &self,
    ) -> terrars::ListRef<CloudwatchEventConnection_InvocationConnectivityParametersRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.invocation_connectivity_parameters", self.extract_ref()),
        )
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
            data: std::cell::RefCell::new(CloudwatchEventConnectionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                authorization_type: self.authorization_type,
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_key_identifier: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                auth_parameters: core::default::Default::default(),
                invocation_connectivity_parameters: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `kms_key_identifier` after provisioning.\n"]
    pub fn kms_key_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_identifier", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auth_parameters` after provisioning.\n"]
    pub fn auth_parameters(&self) -> terrars::ListRef<CloudwatchEventConnection_AuthParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auth_parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `invocation_connectivity_parameters` after provisioning.\n"]
    pub fn invocation_connectivity_parameters(
        &self,
    ) -> terrars::ListRef<CloudwatchEventConnection_InvocationConnectivityParametersRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.invocation_connectivity_parameters", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventConnectionDynamic {
    auth_parameters: std::option::Option<terrars::DynamicBlock<CloudwatchEventConnection_AuthParameters>>,
    invocation_connectivity_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnection_InvocationConnectivityParameters>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventConnection_AuthParameters_ApiKey {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl CloudwatchEventConnection_AuthParameters_ApiKey { }

impl terrars::ToListMappable for CloudwatchEventConnection_AuthParameters_ApiKey {
    type O = terrars::BlockAssignable<CloudwatchEventConnection_AuthParameters_ApiKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnection_AuthParameters_ApiKey {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventConnection_AuthParameters_ApiKey {
    pub fn build(self) -> CloudwatchEventConnection_AuthParameters_ApiKey {
        CloudwatchEventConnection_AuthParameters_ApiKey {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventConnection_AuthParameters_ApiKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnection_AuthParameters_ApiKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnection_AuthParameters_ApiKeyRef {
        CloudwatchEventConnection_AuthParameters_ApiKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnection_AuthParameters_ApiKeyRef {
    #[allow(dead_code)]
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
pub struct CloudwatchEventConnection_AuthParameters_Basic {
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl CloudwatchEventConnection_AuthParameters_Basic { }

impl terrars::ToListMappable for CloudwatchEventConnection_AuthParameters_Basic {
    type O = terrars::BlockAssignable<CloudwatchEventConnection_AuthParameters_Basic>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnection_AuthParameters_Basic {
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventConnection_AuthParameters_Basic {
    pub fn build(self) -> CloudwatchEventConnection_AuthParameters_Basic {
        CloudwatchEventConnection_AuthParameters_Basic {
            password: self.password,
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventConnection_AuthParameters_BasicRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnection_AuthParameters_BasicRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnection_AuthParameters_BasicRef {
        CloudwatchEventConnection_AuthParameters_BasicRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnection_AuthParameters_BasicRef {
    #[allow(dead_code)]
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
pub struct CloudwatchEventConnection_InvocationConnectivityParameters_ResourceParameters {
    resource_configuration_arn: terrars::PrimField<std::string::String>,
}

impl CloudwatchEventConnection_InvocationConnectivityParameters_ResourceParameters { }

impl terrars::ToListMappable for CloudwatchEventConnection_InvocationConnectivityParameters_ResourceParameters {
    type O = terrars::BlockAssignable<CloudwatchEventConnection_InvocationConnectivityParameters_ResourceParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnection_InvocationConnectivityParameters_ResourceParameters {
    #[doc = ""]
    pub resource_configuration_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventConnection_InvocationConnectivityParameters_ResourceParameters {
    pub fn build(self) -> CloudwatchEventConnection_InvocationConnectivityParameters_ResourceParameters {
        CloudwatchEventConnection_InvocationConnectivityParameters_ResourceParameters {
            resource_configuration_arn: self.resource_configuration_arn,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventConnection_InvocationConnectivityParameters_ResourceParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnection_InvocationConnectivityParameters_ResourceParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnection_InvocationConnectivityParameters_ResourceParametersRef {
        CloudwatchEventConnection_InvocationConnectivityParameters_ResourceParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnection_InvocationConnectivityParameters_ResourceParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_association_arn` after provisioning.\n"]
    pub fn resource_association_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_association_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_configuration_arn` after provisioning.\n"]
    pub fn resource_configuration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_configuration_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventConnection_AuthParameters_ConnectivityParametersDynamic {
    resource_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnection_InvocationConnectivityParameters_ResourceParameters>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventConnection_AuthParameters_ConnectivityParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_parameters: std
    ::option
    ::Option<std::vec::Vec<CloudwatchEventConnection_InvocationConnectivityParameters_ResourceParameters>>,
    dynamic: CloudwatchEventConnection_AuthParameters_ConnectivityParametersDynamic,
}

impl CloudwatchEventConnection_AuthParameters_ConnectivityParameters {
    #[doc = "Set the field `resource_parameters`.\n"]
    pub fn set_resource_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudwatchEventConnection_InvocationConnectivityParameters_ResourceParameters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventConnection_AuthParameters_ConnectivityParameters {
    type O = terrars::BlockAssignable<CloudwatchEventConnection_AuthParameters_ConnectivityParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnection_AuthParameters_ConnectivityParameters {}

impl BuildCloudwatchEventConnection_AuthParameters_ConnectivityParameters {
    pub fn build(self) -> CloudwatchEventConnection_AuthParameters_ConnectivityParameters {
        CloudwatchEventConnection_AuthParameters_ConnectivityParameters {
            resource_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventConnection_AuthParameters_ConnectivityParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnection_AuthParameters_ConnectivityParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnection_AuthParameters_ConnectivityParametersRef {
        CloudwatchEventConnection_AuthParameters_ConnectivityParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnection_AuthParameters_ConnectivityParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_parameters` after provisioning.\n"]
    pub fn resource_parameters(
        &self,
    ) -> terrars::ListRef<CloudwatchEventConnection_InvocationConnectivityParameters_ResourceParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Body {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_value_secret: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Body {
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

impl terrars::ToListMappable for CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Body {
    type O = terrars::BlockAssignable<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Body>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Body {}

impl BuildCloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Body {
    pub fn build(self) -> CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Body {
        CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Body {
            is_value_secret: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_BodyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_BodyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_BodyRef {
        CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_BodyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_BodyRef {
    #[allow(dead_code)]
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
pub struct CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Header {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_value_secret: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Header {
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

impl terrars::ToListMappable for CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Header {
    type O = terrars::BlockAssignable<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Header>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Header {}

impl BuildCloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Header {
    pub fn build(self) -> CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Header {
        CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Header {
            is_value_secret: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_HeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_HeaderRef {
        CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_HeaderRef {
    #[allow(dead_code)]
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
pub struct CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryString {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_value_secret: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryString {
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

impl terrars::ToListMappable for CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryString {
    type O = terrars::BlockAssignable<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryString>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryString {}

impl BuildCloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryString {
    pub fn build(self) -> CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryString {
        CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryString {
            is_value_secret: core::default::Default::default(),
            key: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryStringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryStringRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryStringRef {
        CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryStringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryStringRef {
    #[allow(dead_code)]
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
struct CloudwatchEventConnection_AuthParameters_InvocationHttpParametersDynamic {
    body: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Body>>,
    header: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Header>>,
    query_string: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryString>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventConnection_AuthParameters_InvocationHttpParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    body: std::option::Option<std::vec::Vec<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Body>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<std::vec::Vec<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Header>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string: std
    ::option
    ::Option<std::vec::Vec<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryString>>,
    dynamic: CloudwatchEventConnection_AuthParameters_InvocationHttpParametersDynamic,
}

impl CloudwatchEventConnection_AuthParameters_InvocationHttpParameters {
    #[doc = "Set the field `body`.\n"]
    pub fn set_body(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Body>,
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
                        ::BlockAssignable<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Header>,
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
                            CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryString,
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

impl terrars::ToListMappable for CloudwatchEventConnection_AuthParameters_InvocationHttpParameters {
    type O = terrars::BlockAssignable<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnection_AuthParameters_InvocationHttpParameters {}

impl BuildCloudwatchEventConnection_AuthParameters_InvocationHttpParameters {
    pub fn build(self) -> CloudwatchEventConnection_AuthParameters_InvocationHttpParameters {
        CloudwatchEventConnection_AuthParameters_InvocationHttpParameters {
            body: core::default::Default::default(),
            header: core::default::Default::default(),
            query_string: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventConnection_AuthParameters_InvocationHttpParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnection_AuthParameters_InvocationHttpParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnection_AuthParameters_InvocationHttpParametersRef {
        CloudwatchEventConnection_AuthParameters_InvocationHttpParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnection_AuthParameters_InvocationHttpParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> terrars::ListRef<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_BodyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.body", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(
        &self,
    ) -> terrars::ListRef<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_HeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(
        &self,
    ) -> terrars::ListRef<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryStringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_string", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchEventConnection_AuthParameters_Oauth_ClientParameters {
    client_id: terrars::PrimField<std::string::String>,
    client_secret: terrars::PrimField<std::string::String>,
}

impl CloudwatchEventConnection_AuthParameters_Oauth_ClientParameters { }

impl terrars::ToListMappable for CloudwatchEventConnection_AuthParameters_Oauth_ClientParameters {
    type O = terrars::BlockAssignable<CloudwatchEventConnection_AuthParameters_Oauth_ClientParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnection_AuthParameters_Oauth_ClientParameters {
    #[doc = ""]
    pub client_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub client_secret: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventConnection_AuthParameters_Oauth_ClientParameters {
    pub fn build(self) -> CloudwatchEventConnection_AuthParameters_Oauth_ClientParameters {
        CloudwatchEventConnection_AuthParameters_Oauth_ClientParameters {
            client_id: self.client_id,
            client_secret: self.client_secret,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventConnection_AuthParameters_Oauth_ClientParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnection_AuthParameters_Oauth_ClientParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnection_AuthParameters_Oauth_ClientParametersRef {
        CloudwatchEventConnection_AuthParameters_Oauth_ClientParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnection_AuthParameters_Oauth_ClientParametersRef {
    #[allow(dead_code)]
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

#[derive(Serialize, Default)]
struct CloudwatchEventConnection_AuthParameters_Oauth_OauthHttpParametersDynamic {
    body: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Body>>,
    header: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Header>>,
    query_string: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryString>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventConnection_AuthParameters_Oauth_OauthHttpParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    body: std::option::Option<std::vec::Vec<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Body>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<std::vec::Vec<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Header>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string: std
    ::option
    ::Option<std::vec::Vec<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryString>>,
    dynamic: CloudwatchEventConnection_AuthParameters_Oauth_OauthHttpParametersDynamic,
}

impl CloudwatchEventConnection_AuthParameters_Oauth_OauthHttpParameters {
    #[doc = "Set the field `body`.\n"]
    pub fn set_body(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Body>,
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
                        ::BlockAssignable<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_Header>,
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
                            CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryString,
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

impl terrars::ToListMappable for CloudwatchEventConnection_AuthParameters_Oauth_OauthHttpParameters {
    type O = terrars::BlockAssignable<CloudwatchEventConnection_AuthParameters_Oauth_OauthHttpParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnection_AuthParameters_Oauth_OauthHttpParameters {}

impl BuildCloudwatchEventConnection_AuthParameters_Oauth_OauthHttpParameters {
    pub fn build(self) -> CloudwatchEventConnection_AuthParameters_Oauth_OauthHttpParameters {
        CloudwatchEventConnection_AuthParameters_Oauth_OauthHttpParameters {
            body: core::default::Default::default(),
            header: core::default::Default::default(),
            query_string: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventConnection_AuthParameters_Oauth_OauthHttpParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnection_AuthParameters_Oauth_OauthHttpParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnection_AuthParameters_Oauth_OauthHttpParametersRef {
        CloudwatchEventConnection_AuthParameters_Oauth_OauthHttpParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnection_AuthParameters_Oauth_OauthHttpParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> terrars::ListRef<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_BodyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.body", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(
        &self,
    ) -> terrars::ListRef<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_HeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(
        &self,
    ) -> terrars::ListRef<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters_QueryStringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_string", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventConnection_AuthParameters_OauthDynamic {
    client_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnection_AuthParameters_Oauth_ClientParameters>>,
    oauth_http_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnection_AuthParameters_Oauth_OauthHttpParameters>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventConnection_AuthParameters_Oauth {
    authorization_endpoint: terrars::PrimField<std::string::String>,
    http_method: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_parameters: std
    ::option
    ::Option<std::vec::Vec<CloudwatchEventConnection_AuthParameters_Oauth_ClientParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth_http_parameters: std
    ::option
    ::Option<std::vec::Vec<CloudwatchEventConnection_AuthParameters_Oauth_OauthHttpParameters>>,
    dynamic: CloudwatchEventConnection_AuthParameters_OauthDynamic,
}

impl CloudwatchEventConnection_AuthParameters_Oauth {
    #[doc = "Set the field `client_parameters`.\n"]
    pub fn set_client_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchEventConnection_AuthParameters_Oauth_ClientParameters>>,
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
                        terrars::BlockAssignable<CloudwatchEventConnection_AuthParameters_Oauth_OauthHttpParameters>,
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

impl terrars::ToListMappable for CloudwatchEventConnection_AuthParameters_Oauth {
    type O = terrars::BlockAssignable<CloudwatchEventConnection_AuthParameters_Oauth>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnection_AuthParameters_Oauth {
    #[doc = ""]
    pub authorization_endpoint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub http_method: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchEventConnection_AuthParameters_Oauth {
    pub fn build(self) -> CloudwatchEventConnection_AuthParameters_Oauth {
        CloudwatchEventConnection_AuthParameters_Oauth {
            authorization_endpoint: self.authorization_endpoint,
            http_method: self.http_method,
            client_parameters: core::default::Default::default(),
            oauth_http_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventConnection_AuthParameters_OauthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnection_AuthParameters_OauthRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnection_AuthParameters_OauthRef {
        CloudwatchEventConnection_AuthParameters_OauthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnection_AuthParameters_OauthRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<CloudwatchEventConnection_AuthParameters_Oauth_ClientParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth_http_parameters` after provisioning.\n"]
    pub fn oauth_http_parameters(
        &self,
    ) -> terrars::ListRef<CloudwatchEventConnection_AuthParameters_Oauth_OauthHttpParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth_http_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventConnection_AuthParametersDynamic {
    api_key: std::option::Option<terrars::DynamicBlock<CloudwatchEventConnection_AuthParameters_ApiKey>>,
    basic: std::option::Option<terrars::DynamicBlock<CloudwatchEventConnection_AuthParameters_Basic>>,
    connectivity_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnection_AuthParameters_ConnectivityParameters>>,
    invocation_http_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters>>,
    oauth: std::option::Option<terrars::DynamicBlock<CloudwatchEventConnection_AuthParameters_Oauth>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventConnection_AuthParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_key: std::option::Option<std::vec::Vec<CloudwatchEventConnection_AuthParameters_ApiKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic: std::option::Option<std::vec::Vec<CloudwatchEventConnection_AuthParameters_Basic>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connectivity_parameters: std
    ::option
    ::Option<std::vec::Vec<CloudwatchEventConnection_AuthParameters_ConnectivityParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invocation_http_parameters: std
    ::option
    ::Option<std::vec::Vec<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth: std::option::Option<std::vec::Vec<CloudwatchEventConnection_AuthParameters_Oauth>>,
    dynamic: CloudwatchEventConnection_AuthParametersDynamic,
}

impl CloudwatchEventConnection_AuthParameters {
    #[doc = "Set the field `api_key`.\n"]
    pub fn set_api_key(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventConnection_AuthParameters_ApiKey>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventConnection_AuthParameters_Basic>>,
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

    #[doc = "Set the field `connectivity_parameters`.\n"]
    pub fn set_connectivity_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchEventConnection_AuthParameters_ConnectivityParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connectivity_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connectivity_parameters = std::option::Option::Some(d);
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
                        terrars::BlockAssignable<CloudwatchEventConnection_AuthParameters_InvocationHttpParameters>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchEventConnection_AuthParameters_Oauth>>,
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

impl terrars::ToListMappable for CloudwatchEventConnection_AuthParameters {
    type O = terrars::BlockAssignable<CloudwatchEventConnection_AuthParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnection_AuthParameters {}

impl BuildCloudwatchEventConnection_AuthParameters {
    pub fn build(self) -> CloudwatchEventConnection_AuthParameters {
        CloudwatchEventConnection_AuthParameters {
            api_key: core::default::Default::default(),
            basic: core::default::Default::default(),
            connectivity_parameters: core::default::Default::default(),
            invocation_http_parameters: core::default::Default::default(),
            oauth: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventConnection_AuthParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnection_AuthParametersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchEventConnection_AuthParametersRef {
        CloudwatchEventConnection_AuthParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnection_AuthParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_key` after provisioning.\n"]
    pub fn api_key(&self) -> terrars::ListRef<CloudwatchEventConnection_AuthParameters_ApiKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_key", self.base))
    }

    #[doc = "Get a reference to the value of field `basic` after provisioning.\n"]
    pub fn basic(&self) -> terrars::ListRef<CloudwatchEventConnection_AuthParameters_BasicRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic", self.base))
    }

    #[doc = "Get a reference to the value of field `connectivity_parameters` after provisioning.\n"]
    pub fn connectivity_parameters(
        &self,
    ) -> terrars::ListRef<CloudwatchEventConnection_AuthParameters_ConnectivityParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connectivity_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `invocation_http_parameters` after provisioning.\n"]
    pub fn invocation_http_parameters(
        &self,
    ) -> terrars::ListRef<CloudwatchEventConnection_AuthParameters_InvocationHttpParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.invocation_http_parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth` after provisioning.\n"]
    pub fn oauth(&self) -> terrars::ListRef<CloudwatchEventConnection_AuthParameters_OauthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchEventConnection_InvocationConnectivityParametersDynamic {
    resource_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchEventConnection_InvocationConnectivityParameters_ResourceParameters>>,
}

#[derive(Serialize)]
pub struct CloudwatchEventConnection_InvocationConnectivityParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_parameters: std
    ::option
    ::Option<std::vec::Vec<CloudwatchEventConnection_InvocationConnectivityParameters_ResourceParameters>>,
    dynamic: CloudwatchEventConnection_InvocationConnectivityParametersDynamic,
}

impl CloudwatchEventConnection_InvocationConnectivityParameters {
    #[doc = "Set the field `resource_parameters`.\n"]
    pub fn set_resource_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudwatchEventConnection_InvocationConnectivityParameters_ResourceParameters,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchEventConnection_InvocationConnectivityParameters {
    type O = terrars::BlockAssignable<CloudwatchEventConnection_InvocationConnectivityParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchEventConnection_InvocationConnectivityParameters {}

impl BuildCloudwatchEventConnection_InvocationConnectivityParameters {
    pub fn build(self) -> CloudwatchEventConnection_InvocationConnectivityParameters {
        CloudwatchEventConnection_InvocationConnectivityParameters {
            resource_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchEventConnection_InvocationConnectivityParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchEventConnection_InvocationConnectivityParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchEventConnection_InvocationConnectivityParametersRef {
        CloudwatchEventConnection_InvocationConnectivityParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchEventConnection_InvocationConnectivityParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_parameters` after provisioning.\n"]
    pub fn resource_parameters(
        &self,
    ) -> terrars::ListRef<CloudwatchEventConnection_InvocationConnectivityParameters_ResourceParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_parameters", self.base))
    }
}
