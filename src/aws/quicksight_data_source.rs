#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct QuicksightDataSourceData_ {
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
    aws_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    data_source_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credentials: std::option::Option<std::vec::Vec<QuicksightDataSource_Credentials>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission: std::option::Option<std::vec::Vec<QuicksightDataSource_Permission>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_properties: std::option::Option<std::vec::Vec<QuicksightDataSource_SslProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_connection_properties: std::option::Option<std::vec::Vec<QuicksightDataSource_VpcConnectionProperties>>,
    dynamic: QuicksightDataSourceDynamic,
}

struct QuicksightDataSource_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<QuicksightDataSourceData_>,
}

#[derive(Clone)]
pub struct QuicksightDataSource(std::rc::Rc<QuicksightDataSource_>);

impl QuicksightDataSource {
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

    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().aws_account_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `credentials`.\n"]
    pub fn set_credentials(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Credentials>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.credentials = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `permission`.\n"]
    pub fn set_permission(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Permission>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().permission = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.permission = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ssl_properties`.\n"]
    pub fn set_ssl_properties(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_SslProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ssl_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ssl_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vpc_connection_properties`.\n"]
    pub fn set_vpc_connection_properties(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_VpcConnectionProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_connection_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_connection_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(&self) -> terrars::ListRef<QuicksightDataSource_CredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credentials", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<QuicksightDataSource_ParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_properties` after provisioning.\n"]
    pub fn ssl_properties(&self) -> terrars::ListRef<QuicksightDataSource_SslPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_connection_properties` after provisioning.\n"]
    pub fn vpc_connection_properties(&self) -> terrars::ListRef<QuicksightDataSource_VpcConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_connection_properties", self.extract_ref()))
    }
}

impl terrars::Referable for QuicksightDataSource {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for QuicksightDataSource { }

impl terrars::ToListMappable for QuicksightDataSource {
    type O = terrars::ListRef<QuicksightDataSourceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for QuicksightDataSource_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_quicksight_data_source".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildQuicksightDataSource {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub data_source_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSource {
    pub fn build(self, stack: &mut terrars::Stack) -> QuicksightDataSource {
        let out = QuicksightDataSource(std::rc::Rc::new(QuicksightDataSource_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(QuicksightDataSourceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                aws_account_id: core::default::Default::default(),
                data_source_id: self.data_source_id,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                credentials: core::default::Default::default(),
                parameters: core::default::Default::default(),
                permission: core::default::Default::default(),
                ssl_properties: core::default::Default::default(),
                vpc_connection_properties: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct QuicksightDataSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl QuicksightDataSourceRef {
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

    #[doc = "Get a reference to the value of field `aws_account_id` after provisioning.\n"]
    pub fn aws_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_source_id` after provisioning.\n"]
    pub fn data_source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credentials` after provisioning.\n"]
    pub fn credentials(&self) -> terrars::ListRef<QuicksightDataSource_CredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credentials", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<QuicksightDataSource_ParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_properties` after provisioning.\n"]
    pub fn ssl_properties(&self) -> terrars::ListRef<QuicksightDataSource_SslPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_connection_properties` after provisioning.\n"]
    pub fn vpc_connection_properties(&self) -> terrars::ListRef<QuicksightDataSource_VpcConnectionPropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_connection_properties", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct QuicksightDataSourceDynamic {
    credentials: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Credentials>>,
    parameters: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters>>,
    permission: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Permission>>,
    ssl_properties: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_SslProperties>>,
    vpc_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSource_VpcConnectionProperties>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Credentials_CredentialPair {
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSource_Credentials_CredentialPair { }

impl terrars::ToListMappable for QuicksightDataSource_Credentials_CredentialPair {
    type O = terrars::BlockAssignable<QuicksightDataSource_Credentials_CredentialPair>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Credentials_CredentialPair {
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSource_Credentials_CredentialPair {
    pub fn build(self) -> QuicksightDataSource_Credentials_CredentialPair {
        QuicksightDataSource_Credentials_CredentialPair {
            password: self.password,
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Credentials_CredentialPairRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Credentials_CredentialPairRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSource_Credentials_CredentialPairRef {
        QuicksightDataSource_Credentials_CredentialPairRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Credentials_CredentialPairRef {
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

#[derive(Serialize, Default)]
struct QuicksightDataSource_CredentialsDynamic {
    credential_pair: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Credentials_CredentialPair>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Credentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_source_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credential_pair: std::option::Option<std::vec::Vec<QuicksightDataSource_Credentials_CredentialPair>>,
    dynamic: QuicksightDataSource_CredentialsDynamic,
}

impl QuicksightDataSource_Credentials {
    #[doc = "Set the field `copy_source_arn`.\n"]
    pub fn set_copy_source_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.copy_source_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_arn`.\n"]
    pub fn set_secret_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `credential_pair`.\n"]
    pub fn set_credential_pair(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Credentials_CredentialPair>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.credential_pair = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.credential_pair = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSource_Credentials {
    type O = terrars::BlockAssignable<QuicksightDataSource_Credentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Credentials {}

impl BuildQuicksightDataSource_Credentials {
    pub fn build(self) -> QuicksightDataSource_Credentials {
        QuicksightDataSource_Credentials {
            copy_source_arn: core::default::Default::default(),
            secret_arn: core::default::Default::default(),
            credential_pair: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_CredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_CredentialsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_CredentialsRef {
        QuicksightDataSource_CredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_CredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `copy_source_arn` after provisioning.\n"]
    pub fn copy_source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_source_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `credential_pair` after provisioning.\n"]
    pub fn credential_pair(&self) -> terrars::ListRef<QuicksightDataSource_Credentials_CredentialPairRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credential_pair", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_AmazonElasticsearch {
    domain: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSource_Parameters_AmazonElasticsearch { }

impl terrars::ToListMappable for QuicksightDataSource_Parameters_AmazonElasticsearch {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_AmazonElasticsearch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_AmazonElasticsearch {
    #[doc = ""]
    pub domain: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSource_Parameters_AmazonElasticsearch {
    pub fn build(self) -> QuicksightDataSource_Parameters_AmazonElasticsearch {
        QuicksightDataSource_Parameters_AmazonElasticsearch { domain: self.domain }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_AmazonElasticsearchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_AmazonElasticsearchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSource_Parameters_AmazonElasticsearchRef {
        QuicksightDataSource_Parameters_AmazonElasticsearchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_AmazonElasticsearchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_Athena {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    work_group: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl QuicksightDataSource_Parameters_Athena {
    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `work_group`.\n"]
    pub fn set_work_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.work_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSource_Parameters_Athena {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_Athena>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_Athena {}

impl BuildQuicksightDataSource_Parameters_Athena {
    pub fn build(self) -> QuicksightDataSource_Parameters_Athena {
        QuicksightDataSource_Parameters_Athena {
            role_arn: core::default::Default::default(),
            work_group: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_AthenaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_AthenaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_Parameters_AthenaRef {
        QuicksightDataSource_Parameters_AthenaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_AthenaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `work_group` after provisioning.\n"]
    pub fn work_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.work_group", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_Aurora {
    database: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSource_Parameters_Aurora { }

impl terrars::ToListMappable for QuicksightDataSource_Parameters_Aurora {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_Aurora>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_Aurora {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSource_Parameters_Aurora {
    pub fn build(self) -> QuicksightDataSource_Parameters_Aurora {
        QuicksightDataSource_Parameters_Aurora {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_AuroraRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_AuroraRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_Parameters_AuroraRef {
        QuicksightDataSource_Parameters_AuroraRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_AuroraRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_AuroraPostgresql {
    database: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSource_Parameters_AuroraPostgresql { }

impl terrars::ToListMappable for QuicksightDataSource_Parameters_AuroraPostgresql {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_AuroraPostgresql>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_AuroraPostgresql {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSource_Parameters_AuroraPostgresql {
    pub fn build(self) -> QuicksightDataSource_Parameters_AuroraPostgresql {
        QuicksightDataSource_Parameters_AuroraPostgresql {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_AuroraPostgresqlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_AuroraPostgresqlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSource_Parameters_AuroraPostgresqlRef {
        QuicksightDataSource_Parameters_AuroraPostgresqlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_AuroraPostgresqlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_AwsIotAnalytics {
    data_set_name: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSource_Parameters_AwsIotAnalytics { }

impl terrars::ToListMappable for QuicksightDataSource_Parameters_AwsIotAnalytics {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_AwsIotAnalytics>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_AwsIotAnalytics {
    #[doc = ""]
    pub data_set_name: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSource_Parameters_AwsIotAnalytics {
    pub fn build(self) -> QuicksightDataSource_Parameters_AwsIotAnalytics {
        QuicksightDataSource_Parameters_AwsIotAnalytics { data_set_name: self.data_set_name }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_AwsIotAnalyticsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_AwsIotAnalyticsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSource_Parameters_AwsIotAnalyticsRef {
        QuicksightDataSource_Parameters_AwsIotAnalyticsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_AwsIotAnalyticsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_set_name` after provisioning.\n"]
    pub fn data_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_set_name", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_Databricks {
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
    sql_endpoint_path: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSource_Parameters_Databricks { }

impl terrars::ToListMappable for QuicksightDataSource_Parameters_Databricks {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_Databricks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_Databricks {
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
    #[doc = ""]
    pub sql_endpoint_path: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSource_Parameters_Databricks {
    pub fn build(self) -> QuicksightDataSource_Parameters_Databricks {
        QuicksightDataSource_Parameters_Databricks {
            host: self.host,
            port: self.port,
            sql_endpoint_path: self.sql_endpoint_path,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_DatabricksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_DatabricksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_Parameters_DatabricksRef {
        QuicksightDataSource_Parameters_DatabricksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_DatabricksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_endpoint_path` after provisioning.\n"]
    pub fn sql_endpoint_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sql_endpoint_path", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_Jira {
    site_base_url: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSource_Parameters_Jira { }

impl terrars::ToListMappable for QuicksightDataSource_Parameters_Jira {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_Jira>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_Jira {
    #[doc = ""]
    pub site_base_url: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSource_Parameters_Jira {
    pub fn build(self) -> QuicksightDataSource_Parameters_Jira {
        QuicksightDataSource_Parameters_Jira { site_base_url: self.site_base_url }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_JiraRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_JiraRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_Parameters_JiraRef {
        QuicksightDataSource_Parameters_JiraRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_JiraRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `site_base_url` after provisioning.\n"]
    pub fn site_base_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_base_url", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_MariaDb {
    database: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSource_Parameters_MariaDb { }

impl terrars::ToListMappable for QuicksightDataSource_Parameters_MariaDb {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_MariaDb>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_MariaDb {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSource_Parameters_MariaDb {
    pub fn build(self) -> QuicksightDataSource_Parameters_MariaDb {
        QuicksightDataSource_Parameters_MariaDb {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_MariaDbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_MariaDbRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_Parameters_MariaDbRef {
        QuicksightDataSource_Parameters_MariaDbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_MariaDbRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_Mysql {
    database: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSource_Parameters_Mysql { }

impl terrars::ToListMappable for QuicksightDataSource_Parameters_Mysql {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_Mysql>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_Mysql {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSource_Parameters_Mysql {
    pub fn build(self) -> QuicksightDataSource_Parameters_Mysql {
        QuicksightDataSource_Parameters_Mysql {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_MysqlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_MysqlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_Parameters_MysqlRef {
        QuicksightDataSource_Parameters_MysqlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_MysqlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_Oracle {
    database: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSource_Parameters_Oracle { }

impl terrars::ToListMappable for QuicksightDataSource_Parameters_Oracle {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_Oracle>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_Oracle {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSource_Parameters_Oracle {
    pub fn build(self) -> QuicksightDataSource_Parameters_Oracle {
        QuicksightDataSource_Parameters_Oracle {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_OracleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_OracleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_Parameters_OracleRef {
        QuicksightDataSource_Parameters_OracleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_OracleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_Postgresql {
    database: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSource_Parameters_Postgresql { }

impl terrars::ToListMappable for QuicksightDataSource_Parameters_Postgresql {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_Postgresql>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_Postgresql {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSource_Parameters_Postgresql {
    pub fn build(self) -> QuicksightDataSource_Parameters_Postgresql {
        QuicksightDataSource_Parameters_Postgresql {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_PostgresqlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_PostgresqlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_Parameters_PostgresqlRef {
        QuicksightDataSource_Parameters_PostgresqlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_PostgresqlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_Presto {
    catalog: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSource_Parameters_Presto { }

impl terrars::ToListMappable for QuicksightDataSource_Parameters_Presto {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_Presto>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_Presto {
    #[doc = ""]
    pub catalog: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSource_Parameters_Presto {
    pub fn build(self) -> QuicksightDataSource_Parameters_Presto {
        QuicksightDataSource_Parameters_Presto {
            catalog: self.catalog,
            host: self.host,
            port: self.port,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_PrestoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_PrestoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_Parameters_PrestoRef {
        QuicksightDataSource_Parameters_PrestoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_PrestoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `catalog` after provisioning.\n"]
    pub fn catalog(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_Rds {
    database: terrars::PrimField<std::string::String>,
    instance_id: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSource_Parameters_Rds { }

impl terrars::ToListMappable for QuicksightDataSource_Parameters_Rds {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_Rds>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_Rds {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub instance_id: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSource_Parameters_Rds {
    pub fn build(self) -> QuicksightDataSource_Parameters_Rds {
        QuicksightDataSource_Parameters_Rds {
            database: self.database,
            instance_id: self.instance_id,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_RdsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_RdsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_Parameters_RdsRef {
        QuicksightDataSource_Parameters_RdsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_RdsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_Redshift {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_id: std::option::Option<terrars::PrimField<std::string::String>>,
    database: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl QuicksightDataSource_Parameters_Redshift {
    #[doc = "Set the field `cluster_id`.\n"]
    pub fn set_cluster_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host`.\n"]
    pub fn set_host(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSource_Parameters_Redshift {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_Redshift>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_Redshift {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSource_Parameters_Redshift {
    pub fn build(self) -> QuicksightDataSource_Parameters_Redshift {
        QuicksightDataSource_Parameters_Redshift {
            cluster_id: core::default::Default::default(),
            database: self.database,
            host: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_RedshiftRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_RedshiftRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_Parameters_RedshiftRef {
        QuicksightDataSource_Parameters_RedshiftRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_RedshiftRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.base))
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_S3_ManifestFileLocation {
    bucket: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSource_Parameters_S3_ManifestFileLocation { }

impl terrars::ToListMappable for QuicksightDataSource_Parameters_S3_ManifestFileLocation {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_S3_ManifestFileLocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_S3_ManifestFileLocation {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSource_Parameters_S3_ManifestFileLocation {
    pub fn build(self) -> QuicksightDataSource_Parameters_S3_ManifestFileLocation {
        QuicksightDataSource_Parameters_S3_ManifestFileLocation {
            bucket: self.bucket,
            key: self.key,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_S3_ManifestFileLocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_S3_ManifestFileLocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSource_Parameters_S3_ManifestFileLocationRef {
        QuicksightDataSource_Parameters_S3_ManifestFileLocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_S3_ManifestFileLocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize, Default)]
struct QuicksightDataSource_Parameters_S3Dynamic {
    manifest_file_location: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_S3_ManifestFileLocation>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_S3 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manifest_file_location: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_S3_ManifestFileLocation>>,
    dynamic: QuicksightDataSource_Parameters_S3Dynamic,
}

impl QuicksightDataSource_Parameters_S3 {
    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `manifest_file_location`.\n"]
    pub fn set_manifest_file_location(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_S3_ManifestFileLocation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.manifest_file_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.manifest_file_location = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSource_Parameters_S3 {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_S3 {}

impl BuildQuicksightDataSource_Parameters_S3 {
    pub fn build(self) -> QuicksightDataSource_Parameters_S3 {
        QuicksightDataSource_Parameters_S3 {
            role_arn: core::default::Default::default(),
            manifest_file_location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_S3Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_Parameters_S3Ref {
        QuicksightDataSource_Parameters_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `manifest_file_location` after provisioning.\n"]
    pub fn manifest_file_location(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_S3_ManifestFileLocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.manifest_file_location", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_ServiceNow {
    site_base_url: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSource_Parameters_ServiceNow { }

impl terrars::ToListMappable for QuicksightDataSource_Parameters_ServiceNow {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_ServiceNow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_ServiceNow {
    #[doc = ""]
    pub site_base_url: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSource_Parameters_ServiceNow {
    pub fn build(self) -> QuicksightDataSource_Parameters_ServiceNow {
        QuicksightDataSource_Parameters_ServiceNow { site_base_url: self.site_base_url }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_ServiceNowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_ServiceNowRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_Parameters_ServiceNowRef {
        QuicksightDataSource_Parameters_ServiceNowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_ServiceNowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `site_base_url` after provisioning.\n"]
    pub fn site_base_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_base_url", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_Snowflake {
    database: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    warehouse: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSource_Parameters_Snowflake { }

impl terrars::ToListMappable for QuicksightDataSource_Parameters_Snowflake {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_Snowflake>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_Snowflake {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub warehouse: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSource_Parameters_Snowflake {
    pub fn build(self) -> QuicksightDataSource_Parameters_Snowflake {
        QuicksightDataSource_Parameters_Snowflake {
            database: self.database,
            host: self.host,
            warehouse: self.warehouse,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_SnowflakeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_SnowflakeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_Parameters_SnowflakeRef {
        QuicksightDataSource_Parameters_SnowflakeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_SnowflakeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `warehouse` after provisioning.\n"]
    pub fn warehouse(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warehouse", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_Spark {
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSource_Parameters_Spark { }

impl terrars::ToListMappable for QuicksightDataSource_Parameters_Spark {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_Spark>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_Spark {
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSource_Parameters_Spark {
    pub fn build(self) -> QuicksightDataSource_Parameters_Spark {
        QuicksightDataSource_Parameters_Spark {
            host: self.host,
            port: self.port,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_SparkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_SparkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_Parameters_SparkRef {
        QuicksightDataSource_Parameters_SparkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_SparkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_SqlServer {
    database: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSource_Parameters_SqlServer { }

impl terrars::ToListMappable for QuicksightDataSource_Parameters_SqlServer {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_SqlServer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_SqlServer {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSource_Parameters_SqlServer {
    pub fn build(self) -> QuicksightDataSource_Parameters_SqlServer {
        QuicksightDataSource_Parameters_SqlServer {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_SqlServerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_SqlServerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_Parameters_SqlServerRef {
        QuicksightDataSource_Parameters_SqlServerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_SqlServerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_Teradata {
    database: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSource_Parameters_Teradata { }

impl terrars::ToListMappable for QuicksightDataSource_Parameters_Teradata {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_Teradata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_Teradata {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSource_Parameters_Teradata {
    pub fn build(self) -> QuicksightDataSource_Parameters_Teradata {
        QuicksightDataSource_Parameters_Teradata {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_TeradataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_TeradataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_Parameters_TeradataRef {
        QuicksightDataSource_Parameters_TeradataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_TeradataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `database` after provisioning.\n"]
    pub fn database(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.database", self.base))
    }

    #[doc = "Get a reference to the value of field `host` after provisioning.\n"]
    pub fn host(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters_Twitter {
    max_rows: terrars::PrimField<f64>,
    query: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSource_Parameters_Twitter { }

impl terrars::ToListMappable for QuicksightDataSource_Parameters_Twitter {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters_Twitter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters_Twitter {
    #[doc = ""]
    pub max_rows: terrars::PrimField<f64>,
    #[doc = ""]
    pub query: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSource_Parameters_Twitter {
    pub fn build(self) -> QuicksightDataSource_Parameters_Twitter {
        QuicksightDataSource_Parameters_Twitter {
            max_rows: self.max_rows,
            query: self.query,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_Parameters_TwitterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_Parameters_TwitterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_Parameters_TwitterRef {
        QuicksightDataSource_Parameters_TwitterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_Parameters_TwitterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_rows` after provisioning.\n"]
    pub fn max_rows(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_rows", self.base))
    }

    #[doc = "Get a reference to the value of field `query` after provisioning.\n"]
    pub fn query(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.query", self.base))
    }
}

#[derive(Serialize, Default)]
struct QuicksightDataSource_ParametersDynamic {
    amazon_elasticsearch: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_AmazonElasticsearch>>,
    athena: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_Athena>>,
    aurora: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_Aurora>>,
    aurora_postgresql: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_AuroraPostgresql>>,
    aws_iot_analytics: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_AwsIotAnalytics>>,
    databricks: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_Databricks>>,
    jira: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_Jira>>,
    maria_db: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_MariaDb>>,
    mysql: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_Mysql>>,
    oracle: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_Oracle>>,
    postgresql: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_Postgresql>>,
    presto: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_Presto>>,
    rds: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_Rds>>,
    redshift: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_Redshift>>,
    s3: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_S3>>,
    service_now: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_ServiceNow>>,
    snowflake: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_Snowflake>>,
    spark: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_Spark>>,
    sql_server: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_SqlServer>>,
    teradata: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_Teradata>>,
    twitter: std::option::Option<terrars::DynamicBlock<QuicksightDataSource_Parameters_Twitter>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Parameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_elasticsearch: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_AmazonElasticsearch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    athena: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_Athena>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aurora: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_Aurora>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aurora_postgresql: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_AuroraPostgresql>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_iot_analytics: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_AwsIotAnalytics>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    databricks: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_Databricks>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jira: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_Jira>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maria_db: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_MariaDb>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_Mysql>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_Oracle>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_Postgresql>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    presto: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_Presto>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rds: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_Rds>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redshift: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_Redshift>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_S3>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_now: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_ServiceNow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snowflake: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_Snowflake>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_Spark>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_server: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_SqlServer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    teradata: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_Teradata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    twitter: std::option::Option<std::vec::Vec<QuicksightDataSource_Parameters_Twitter>>,
    dynamic: QuicksightDataSource_ParametersDynamic,
}

impl QuicksightDataSource_Parameters {
    #[doc = "Set the field `amazon_elasticsearch`.\n"]
    pub fn set_amazon_elasticsearch(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_AmazonElasticsearch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amazon_elasticsearch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amazon_elasticsearch = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `athena`.\n"]
    pub fn set_athena(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_Athena>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.athena = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.athena = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aurora`.\n"]
    pub fn set_aurora(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_Aurora>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aurora = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aurora = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aurora_postgresql`.\n"]
    pub fn set_aurora_postgresql(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_AuroraPostgresql>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aurora_postgresql = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aurora_postgresql = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aws_iot_analytics`.\n"]
    pub fn set_aws_iot_analytics(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_AwsIotAnalytics>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_iot_analytics = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_iot_analytics = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `databricks`.\n"]
    pub fn set_databricks(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_Databricks>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.databricks = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.databricks = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `jira`.\n"]
    pub fn set_jira(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_Jira>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.jira = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.jira = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maria_db`.\n"]
    pub fn set_maria_db(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_MariaDb>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maria_db = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maria_db = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `mysql`.\n"]
    pub fn set_mysql(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_Mysql>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mysql = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mysql = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oracle`.\n"]
    pub fn set_oracle(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_Oracle>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oracle = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oracle = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `postgresql`.\n"]
    pub fn set_postgresql(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_Postgresql>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.postgresql = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.postgresql = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `presto`.\n"]
    pub fn set_presto(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_Presto>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.presto = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.presto = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rds`.\n"]
    pub fn set_rds(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_Rds>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rds = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rds = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redshift`.\n"]
    pub fn set_redshift(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_Redshift>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redshift = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redshift = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_S3>>,
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

    #[doc = "Set the field `service_now`.\n"]
    pub fn set_service_now(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_ServiceNow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_now = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_now = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `snowflake`.\n"]
    pub fn set_snowflake(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_Snowflake>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.snowflake = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.snowflake = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spark`.\n"]
    pub fn set_spark(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_Spark>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.spark = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.spark = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sql_server`.\n"]
    pub fn set_sql_server(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_SqlServer>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sql_server = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sql_server = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `teradata`.\n"]
    pub fn set_teradata(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_Teradata>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.teradata = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.teradata = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `twitter`.\n"]
    pub fn set_twitter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSource_Parameters_Twitter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.twitter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.twitter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSource_Parameters {
    type O = terrars::BlockAssignable<QuicksightDataSource_Parameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Parameters {}

impl BuildQuicksightDataSource_Parameters {
    pub fn build(self) -> QuicksightDataSource_Parameters {
        QuicksightDataSource_Parameters {
            amazon_elasticsearch: core::default::Default::default(),
            athena: core::default::Default::default(),
            aurora: core::default::Default::default(),
            aurora_postgresql: core::default::Default::default(),
            aws_iot_analytics: core::default::Default::default(),
            databricks: core::default::Default::default(),
            jira: core::default::Default::default(),
            maria_db: core::default::Default::default(),
            mysql: core::default::Default::default(),
            oracle: core::default::Default::default(),
            postgresql: core::default::Default::default(),
            presto: core::default::Default::default(),
            rds: core::default::Default::default(),
            redshift: core::default::Default::default(),
            s3: core::default::Default::default(),
            service_now: core::default::Default::default(),
            snowflake: core::default::Default::default(),
            spark: core::default::Default::default(),
            sql_server: core::default::Default::default(),
            teradata: core::default::Default::default(),
            twitter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_ParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_ParametersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_ParametersRef {
        QuicksightDataSource_ParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_ParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amazon_elasticsearch` after provisioning.\n"]
    pub fn amazon_elasticsearch(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_AmazonElasticsearchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amazon_elasticsearch", self.base))
    }

    #[doc = "Get a reference to the value of field `athena` after provisioning.\n"]
    pub fn athena(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_AthenaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.athena", self.base))
    }

    #[doc = "Get a reference to the value of field `aurora` after provisioning.\n"]
    pub fn aurora(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_AuroraRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aurora", self.base))
    }

    #[doc = "Get a reference to the value of field `aurora_postgresql` after provisioning.\n"]
    pub fn aurora_postgresql(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_AuroraPostgresqlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aurora_postgresql", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_iot_analytics` after provisioning.\n"]
    pub fn aws_iot_analytics(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_AwsIotAnalyticsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_iot_analytics", self.base))
    }

    #[doc = "Get a reference to the value of field `databricks` after provisioning.\n"]
    pub fn databricks(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_DatabricksRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.databricks", self.base))
    }

    #[doc = "Get a reference to the value of field `jira` after provisioning.\n"]
    pub fn jira(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_JiraRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jira", self.base))
    }

    #[doc = "Get a reference to the value of field `maria_db` after provisioning.\n"]
    pub fn maria_db(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_MariaDbRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maria_db", self.base))
    }

    #[doc = "Get a reference to the value of field `mysql` after provisioning.\n"]
    pub fn mysql(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_MysqlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql", self.base))
    }

    #[doc = "Get a reference to the value of field `oracle` after provisioning.\n"]
    pub fn oracle(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_OracleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle", self.base))
    }

    #[doc = "Get a reference to the value of field `postgresql` after provisioning.\n"]
    pub fn postgresql(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_PostgresqlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql", self.base))
    }

    #[doc = "Get a reference to the value of field `presto` after provisioning.\n"]
    pub fn presto(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_PrestoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.presto", self.base))
    }

    #[doc = "Get a reference to the value of field `rds` after provisioning.\n"]
    pub fn rds(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_RdsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rds", self.base))
    }

    #[doc = "Get a reference to the value of field `redshift` after provisioning.\n"]
    pub fn redshift(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_RedshiftRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `service_now` after provisioning.\n"]
    pub fn service_now(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_ServiceNowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_now", self.base))
    }

    #[doc = "Get a reference to the value of field `snowflake` after provisioning.\n"]
    pub fn snowflake(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_SnowflakeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snowflake", self.base))
    }

    #[doc = "Get a reference to the value of field `spark` after provisioning.\n"]
    pub fn spark(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_SparkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_server` after provisioning.\n"]
    pub fn sql_server(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_SqlServerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sql_server", self.base))
    }

    #[doc = "Get a reference to the value of field `teradata` after provisioning.\n"]
    pub fn teradata(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_TeradataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.teradata", self.base))
    }

    #[doc = "Get a reference to the value of field `twitter` after provisioning.\n"]
    pub fn twitter(&self) -> terrars::ListRef<QuicksightDataSource_Parameters_TwitterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.twitter", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_Permission {
    actions: terrars::SetField<terrars::PrimField<std::string::String>>,
    principal: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSource_Permission { }

impl terrars::ToListMappable for QuicksightDataSource_Permission {
    type O = terrars::BlockAssignable<QuicksightDataSource_Permission>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_Permission {
    #[doc = ""]
    pub actions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub principal: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSource_Permission {
    pub fn build(self) -> QuicksightDataSource_Permission {
        QuicksightDataSource_Permission {
            actions: self.actions,
            principal: self.principal,
        }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_PermissionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_PermissionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_PermissionRef {
        QuicksightDataSource_PermissionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_PermissionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_SslProperties {
    disable_ssl: terrars::PrimField<bool>,
}

impl QuicksightDataSource_SslProperties { }

impl terrars::ToListMappable for QuicksightDataSource_SslProperties {
    type O = terrars::BlockAssignable<QuicksightDataSource_SslProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_SslProperties {
    #[doc = ""]
    pub disable_ssl: terrars::PrimField<bool>,
}

impl BuildQuicksightDataSource_SslProperties {
    pub fn build(self) -> QuicksightDataSource_SslProperties {
        QuicksightDataSource_SslProperties { disable_ssl: self.disable_ssl }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_SslPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_SslPropertiesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSource_SslPropertiesRef {
        QuicksightDataSource_SslPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_SslPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable_ssl` after provisioning.\n"]
    pub fn disable_ssl(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_ssl", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSource_VpcConnectionProperties {
    vpc_connection_arn: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSource_VpcConnectionProperties { }

impl terrars::ToListMappable for QuicksightDataSource_VpcConnectionProperties {
    type O = terrars::BlockAssignable<QuicksightDataSource_VpcConnectionProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSource_VpcConnectionProperties {
    #[doc = ""]
    pub vpc_connection_arn: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSource_VpcConnectionProperties {
    pub fn build(self) -> QuicksightDataSource_VpcConnectionProperties {
        QuicksightDataSource_VpcConnectionProperties { vpc_connection_arn: self.vpc_connection_arn }
    }
}

#[allow(dead_code)]
pub struct QuicksightDataSource_VpcConnectionPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSource_VpcConnectionPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSource_VpcConnectionPropertiesRef {
        QuicksightDataSource_VpcConnectionPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSource_VpcConnectionPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vpc_connection_arn` after provisioning.\n"]
    pub fn vpc_connection_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_connection_arn", self.base))
    }
}
