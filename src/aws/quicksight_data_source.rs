use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct QuicksightDataSourceData {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credentials: std::option::Option<std::vec::Vec<QuicksightDataSourceCredentialsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permission: std::option::Option<std::vec::Vec<QuicksightDataSourcePermissionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_properties: std::option::Option<std::vec::Vec<QuicksightDataSourceSslPropertiesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_connection_properties: std::option::Option<std::vec::Vec<QuicksightDataSourceVpcConnectionPropertiesEl>>,
    dynamic: QuicksightDataSourceDynamic,
}

struct QuicksightDataSource_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<QuicksightDataSourceData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceCredentialsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourcePermissionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceSslPropertiesEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceVpcConnectionPropertiesEl>>,
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
    pub fn credentials(&self) -> terrars::ListRef<QuicksightDataSourceCredentialsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credentials", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<QuicksightDataSourceParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_properties` after provisioning.\n"]
    pub fn ssl_properties(&self) -> terrars::ListRef<QuicksightDataSourceSslPropertiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_connection_properties` after provisioning.\n"]
    pub fn vpc_connection_properties(&self) -> terrars::ListRef<QuicksightDataSourceVpcConnectionPropertiesElRef> {
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
            data: std::cell::RefCell::new(QuicksightDataSourceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                aws_account_id: core::default::Default::default(),
                data_source_id: self.data_source_id,
                id: core::default::Default::default(),
                name: self.name,
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
    pub fn credentials(&self) -> terrars::ListRef<QuicksightDataSourceCredentialsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credentials", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<QuicksightDataSourceParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssl_properties` after provisioning.\n"]
    pub fn ssl_properties(&self) -> terrars::ListRef<QuicksightDataSourceSslPropertiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssl_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_connection_properties` after provisioning.\n"]
    pub fn vpc_connection_properties(&self) -> terrars::ListRef<QuicksightDataSourceVpcConnectionPropertiesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_connection_properties", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSourceCredentialsElCredentialPairEl {
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSourceCredentialsElCredentialPairEl { }

impl terrars::ToListMappable for QuicksightDataSourceCredentialsElCredentialPairEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceCredentialsElCredentialPairEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceCredentialsElCredentialPairEl {
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSourceCredentialsElCredentialPairEl {
    pub fn build(self) -> QuicksightDataSourceCredentialsElCredentialPairEl {
        QuicksightDataSourceCredentialsElCredentialPairEl {
            password: self.password,
            username: self.username,
        }
    }
}

pub struct QuicksightDataSourceCredentialsElCredentialPairElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceCredentialsElCredentialPairElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSourceCredentialsElCredentialPairElRef {
        QuicksightDataSourceCredentialsElCredentialPairElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceCredentialsElCredentialPairElRef {
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
struct QuicksightDataSourceCredentialsElDynamic {
    credential_pair: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceCredentialsElCredentialPairEl>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSourceCredentialsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_source_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credential_pair: std::option::Option<std::vec::Vec<QuicksightDataSourceCredentialsElCredentialPairEl>>,
    dynamic: QuicksightDataSourceCredentialsElDynamic,
}

impl QuicksightDataSourceCredentialsEl {
    #[doc = "Set the field `copy_source_arn`.\n"]
    pub fn set_copy_source_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.copy_source_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `credential_pair`.\n"]
    pub fn set_credential_pair(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceCredentialsElCredentialPairEl>>,
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

impl terrars::ToListMappable for QuicksightDataSourceCredentialsEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceCredentialsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceCredentialsEl {}

impl BuildQuicksightDataSourceCredentialsEl {
    pub fn build(self) -> QuicksightDataSourceCredentialsEl {
        QuicksightDataSourceCredentialsEl {
            copy_source_arn: core::default::Default::default(),
            credential_pair: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct QuicksightDataSourceCredentialsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceCredentialsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSourceCredentialsElRef {
        QuicksightDataSourceCredentialsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceCredentialsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `copy_source_arn` after provisioning.\n"]
    pub fn copy_source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_source_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `credential_pair` after provisioning.\n"]
    pub fn credential_pair(&self) -> terrars::ListRef<QuicksightDataSourceCredentialsElCredentialPairElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credential_pair", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElAmazonElasticsearchEl {
    domain: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSourceParametersElAmazonElasticsearchEl { }

impl terrars::ToListMappable for QuicksightDataSourceParametersElAmazonElasticsearchEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElAmazonElasticsearchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElAmazonElasticsearchEl {
    #[doc = ""]
    pub domain: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSourceParametersElAmazonElasticsearchEl {
    pub fn build(self) -> QuicksightDataSourceParametersElAmazonElasticsearchEl {
        QuicksightDataSourceParametersElAmazonElasticsearchEl { domain: self.domain }
    }
}

pub struct QuicksightDataSourceParametersElAmazonElasticsearchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElAmazonElasticsearchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSourceParametersElAmazonElasticsearchElRef {
        QuicksightDataSourceParametersElAmazonElasticsearchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElAmazonElasticsearchElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElAthenaEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    work_group: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl QuicksightDataSourceParametersElAthenaEl {
    #[doc = "Set the field `work_group`.\n"]
    pub fn set_work_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.work_group = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for QuicksightDataSourceParametersElAthenaEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElAthenaEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElAthenaEl {}

impl BuildQuicksightDataSourceParametersElAthenaEl {
    pub fn build(self) -> QuicksightDataSourceParametersElAthenaEl {
        QuicksightDataSourceParametersElAthenaEl { work_group: core::default::Default::default() }
    }
}

pub struct QuicksightDataSourceParametersElAthenaElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElAthenaElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSourceParametersElAthenaElRef {
        QuicksightDataSourceParametersElAthenaElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElAthenaElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `work_group` after provisioning.\n"]
    pub fn work_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.work_group", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElAuroraEl {
    database: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSourceParametersElAuroraEl { }

impl terrars::ToListMappable for QuicksightDataSourceParametersElAuroraEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElAuroraEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElAuroraEl {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSourceParametersElAuroraEl {
    pub fn build(self) -> QuicksightDataSourceParametersElAuroraEl {
        QuicksightDataSourceParametersElAuroraEl {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}

pub struct QuicksightDataSourceParametersElAuroraElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElAuroraElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSourceParametersElAuroraElRef {
        QuicksightDataSourceParametersElAuroraElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElAuroraElRef {
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
pub struct QuicksightDataSourceParametersElAuroraPostgresqlEl {
    database: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSourceParametersElAuroraPostgresqlEl { }

impl terrars::ToListMappable for QuicksightDataSourceParametersElAuroraPostgresqlEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElAuroraPostgresqlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElAuroraPostgresqlEl {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSourceParametersElAuroraPostgresqlEl {
    pub fn build(self) -> QuicksightDataSourceParametersElAuroraPostgresqlEl {
        QuicksightDataSourceParametersElAuroraPostgresqlEl {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}

pub struct QuicksightDataSourceParametersElAuroraPostgresqlElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElAuroraPostgresqlElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSourceParametersElAuroraPostgresqlElRef {
        QuicksightDataSourceParametersElAuroraPostgresqlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElAuroraPostgresqlElRef {
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
pub struct QuicksightDataSourceParametersElAwsIotAnalyticsEl {
    data_set_name: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSourceParametersElAwsIotAnalyticsEl { }

impl terrars::ToListMappable for QuicksightDataSourceParametersElAwsIotAnalyticsEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElAwsIotAnalyticsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElAwsIotAnalyticsEl {
    #[doc = ""]
    pub data_set_name: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSourceParametersElAwsIotAnalyticsEl {
    pub fn build(self) -> QuicksightDataSourceParametersElAwsIotAnalyticsEl {
        QuicksightDataSourceParametersElAwsIotAnalyticsEl { data_set_name: self.data_set_name }
    }
}

pub struct QuicksightDataSourceParametersElAwsIotAnalyticsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElAwsIotAnalyticsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSourceParametersElAwsIotAnalyticsElRef {
        QuicksightDataSourceParametersElAwsIotAnalyticsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElAwsIotAnalyticsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_set_name` after provisioning.\n"]
    pub fn data_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_set_name", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElJiraEl {
    site_base_url: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSourceParametersElJiraEl { }

impl terrars::ToListMappable for QuicksightDataSourceParametersElJiraEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElJiraEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElJiraEl {
    #[doc = ""]
    pub site_base_url: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSourceParametersElJiraEl {
    pub fn build(self) -> QuicksightDataSourceParametersElJiraEl {
        QuicksightDataSourceParametersElJiraEl { site_base_url: self.site_base_url }
    }
}

pub struct QuicksightDataSourceParametersElJiraElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElJiraElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSourceParametersElJiraElRef {
        QuicksightDataSourceParametersElJiraElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElJiraElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `site_base_url` after provisioning.\n"]
    pub fn site_base_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_base_url", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElMariaDbEl {
    database: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSourceParametersElMariaDbEl { }

impl terrars::ToListMappable for QuicksightDataSourceParametersElMariaDbEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElMariaDbEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElMariaDbEl {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSourceParametersElMariaDbEl {
    pub fn build(self) -> QuicksightDataSourceParametersElMariaDbEl {
        QuicksightDataSourceParametersElMariaDbEl {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}

pub struct QuicksightDataSourceParametersElMariaDbElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElMariaDbElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSourceParametersElMariaDbElRef {
        QuicksightDataSourceParametersElMariaDbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElMariaDbElRef {
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
pub struct QuicksightDataSourceParametersElMysqlEl {
    database: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSourceParametersElMysqlEl { }

impl terrars::ToListMappable for QuicksightDataSourceParametersElMysqlEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElMysqlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElMysqlEl {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSourceParametersElMysqlEl {
    pub fn build(self) -> QuicksightDataSourceParametersElMysqlEl {
        QuicksightDataSourceParametersElMysqlEl {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}

pub struct QuicksightDataSourceParametersElMysqlElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElMysqlElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSourceParametersElMysqlElRef {
        QuicksightDataSourceParametersElMysqlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElMysqlElRef {
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
pub struct QuicksightDataSourceParametersElOracleEl {
    database: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSourceParametersElOracleEl { }

impl terrars::ToListMappable for QuicksightDataSourceParametersElOracleEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElOracleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElOracleEl {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSourceParametersElOracleEl {
    pub fn build(self) -> QuicksightDataSourceParametersElOracleEl {
        QuicksightDataSourceParametersElOracleEl {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}

pub struct QuicksightDataSourceParametersElOracleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElOracleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSourceParametersElOracleElRef {
        QuicksightDataSourceParametersElOracleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElOracleElRef {
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
pub struct QuicksightDataSourceParametersElPostgresqlEl {
    database: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSourceParametersElPostgresqlEl { }

impl terrars::ToListMappable for QuicksightDataSourceParametersElPostgresqlEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElPostgresqlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElPostgresqlEl {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSourceParametersElPostgresqlEl {
    pub fn build(self) -> QuicksightDataSourceParametersElPostgresqlEl {
        QuicksightDataSourceParametersElPostgresqlEl {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}

pub struct QuicksightDataSourceParametersElPostgresqlElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElPostgresqlElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSourceParametersElPostgresqlElRef {
        QuicksightDataSourceParametersElPostgresqlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElPostgresqlElRef {
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
pub struct QuicksightDataSourceParametersElPrestoEl {
    catalog: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSourceParametersElPrestoEl { }

impl terrars::ToListMappable for QuicksightDataSourceParametersElPrestoEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElPrestoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElPrestoEl {
    #[doc = ""]
    pub catalog: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSourceParametersElPrestoEl {
    pub fn build(self) -> QuicksightDataSourceParametersElPrestoEl {
        QuicksightDataSourceParametersElPrestoEl {
            catalog: self.catalog,
            host: self.host,
            port: self.port,
        }
    }
}

pub struct QuicksightDataSourceParametersElPrestoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElPrestoElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSourceParametersElPrestoElRef {
        QuicksightDataSourceParametersElPrestoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElPrestoElRef {
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
pub struct QuicksightDataSourceParametersElRdsEl {
    database: terrars::PrimField<std::string::String>,
    instance_id: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSourceParametersElRdsEl { }

impl terrars::ToListMappable for QuicksightDataSourceParametersElRdsEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElRdsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElRdsEl {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub instance_id: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSourceParametersElRdsEl {
    pub fn build(self) -> QuicksightDataSourceParametersElRdsEl {
        QuicksightDataSourceParametersElRdsEl {
            database: self.database,
            instance_id: self.instance_id,
        }
    }
}

pub struct QuicksightDataSourceParametersElRdsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElRdsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSourceParametersElRdsElRef {
        QuicksightDataSourceParametersElRdsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElRdsElRef {
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
pub struct QuicksightDataSourceParametersElRedshiftEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_id: std::option::Option<terrars::PrimField<std::string::String>>,
    database: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl QuicksightDataSourceParametersElRedshiftEl {
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

impl terrars::ToListMappable for QuicksightDataSourceParametersElRedshiftEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElRedshiftEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElRedshiftEl {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSourceParametersElRedshiftEl {
    pub fn build(self) -> QuicksightDataSourceParametersElRedshiftEl {
        QuicksightDataSourceParametersElRedshiftEl {
            cluster_id: core::default::Default::default(),
            database: self.database,
            host: core::default::Default::default(),
            port: core::default::Default::default(),
        }
    }
}

pub struct QuicksightDataSourceParametersElRedshiftElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElRedshiftElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSourceParametersElRedshiftElRef {
        QuicksightDataSourceParametersElRedshiftElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElRedshiftElRef {
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
pub struct QuicksightDataSourceParametersElS3ElManifestFileLocationEl {
    bucket: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSourceParametersElS3ElManifestFileLocationEl { }

impl terrars::ToListMappable for QuicksightDataSourceParametersElS3ElManifestFileLocationEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElS3ElManifestFileLocationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElS3ElManifestFileLocationEl {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSourceParametersElS3ElManifestFileLocationEl {
    pub fn build(self) -> QuicksightDataSourceParametersElS3ElManifestFileLocationEl {
        QuicksightDataSourceParametersElS3ElManifestFileLocationEl {
            bucket: self.bucket,
            key: self.key,
        }
    }
}

pub struct QuicksightDataSourceParametersElS3ElManifestFileLocationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElS3ElManifestFileLocationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSourceParametersElS3ElManifestFileLocationElRef {
        QuicksightDataSourceParametersElS3ElManifestFileLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElS3ElManifestFileLocationElRef {
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
struct QuicksightDataSourceParametersElS3ElDynamic {
    manifest_file_location: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElS3ElManifestFileLocationEl>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElS3El {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manifest_file_location: std
    ::option
    ::Option<std::vec::Vec<QuicksightDataSourceParametersElS3ElManifestFileLocationEl>>,
    dynamic: QuicksightDataSourceParametersElS3ElDynamic,
}

impl QuicksightDataSourceParametersElS3El {
    #[doc = "Set the field `manifest_file_location`.\n"]
    pub fn set_manifest_file_location(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElS3ElManifestFileLocationEl>>,
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

impl terrars::ToListMappable for QuicksightDataSourceParametersElS3El {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElS3El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElS3El {}

impl BuildQuicksightDataSourceParametersElS3El {
    pub fn build(self) -> QuicksightDataSourceParametersElS3El {
        QuicksightDataSourceParametersElS3El {
            manifest_file_location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct QuicksightDataSourceParametersElS3ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElS3ElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSourceParametersElS3ElRef {
        QuicksightDataSourceParametersElS3ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElS3ElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `manifest_file_location` after provisioning.\n"]
    pub fn manifest_file_location(
        &self,
    ) -> terrars::ListRef<QuicksightDataSourceParametersElS3ElManifestFileLocationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.manifest_file_location", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElServiceNowEl {
    site_base_url: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSourceParametersElServiceNowEl { }

impl terrars::ToListMappable for QuicksightDataSourceParametersElServiceNowEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElServiceNowEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElServiceNowEl {
    #[doc = ""]
    pub site_base_url: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSourceParametersElServiceNowEl {
    pub fn build(self) -> QuicksightDataSourceParametersElServiceNowEl {
        QuicksightDataSourceParametersElServiceNowEl { site_base_url: self.site_base_url }
    }
}

pub struct QuicksightDataSourceParametersElServiceNowElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElServiceNowElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSourceParametersElServiceNowElRef {
        QuicksightDataSourceParametersElServiceNowElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElServiceNowElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `site_base_url` after provisioning.\n"]
    pub fn site_base_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_base_url", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSourceParametersElSnowflakeEl {
    database: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    warehouse: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSourceParametersElSnowflakeEl { }

impl terrars::ToListMappable for QuicksightDataSourceParametersElSnowflakeEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElSnowflakeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElSnowflakeEl {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub warehouse: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSourceParametersElSnowflakeEl {
    pub fn build(self) -> QuicksightDataSourceParametersElSnowflakeEl {
        QuicksightDataSourceParametersElSnowflakeEl {
            database: self.database,
            host: self.host,
            warehouse: self.warehouse,
        }
    }
}

pub struct QuicksightDataSourceParametersElSnowflakeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElSnowflakeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSourceParametersElSnowflakeElRef {
        QuicksightDataSourceParametersElSnowflakeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElSnowflakeElRef {
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
pub struct QuicksightDataSourceParametersElSparkEl {
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSourceParametersElSparkEl { }

impl terrars::ToListMappable for QuicksightDataSourceParametersElSparkEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElSparkEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElSparkEl {
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSourceParametersElSparkEl {
    pub fn build(self) -> QuicksightDataSourceParametersElSparkEl {
        QuicksightDataSourceParametersElSparkEl {
            host: self.host,
            port: self.port,
        }
    }
}

pub struct QuicksightDataSourceParametersElSparkElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElSparkElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSourceParametersElSparkElRef {
        QuicksightDataSourceParametersElSparkElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElSparkElRef {
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
pub struct QuicksightDataSourceParametersElSqlServerEl {
    database: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSourceParametersElSqlServerEl { }

impl terrars::ToListMappable for QuicksightDataSourceParametersElSqlServerEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElSqlServerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElSqlServerEl {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSourceParametersElSqlServerEl {
    pub fn build(self) -> QuicksightDataSourceParametersElSqlServerEl {
        QuicksightDataSourceParametersElSqlServerEl {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}

pub struct QuicksightDataSourceParametersElSqlServerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElSqlServerElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSourceParametersElSqlServerElRef {
        QuicksightDataSourceParametersElSqlServerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElSqlServerElRef {
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
pub struct QuicksightDataSourceParametersElTeradataEl {
    database: terrars::PrimField<std::string::String>,
    host: terrars::PrimField<std::string::String>,
    port: terrars::PrimField<f64>,
}

impl QuicksightDataSourceParametersElTeradataEl { }

impl terrars::ToListMappable for QuicksightDataSourceParametersElTeradataEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElTeradataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElTeradataEl {
    #[doc = ""]
    pub database: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub host: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
}

impl BuildQuicksightDataSourceParametersElTeradataEl {
    pub fn build(self) -> QuicksightDataSourceParametersElTeradataEl {
        QuicksightDataSourceParametersElTeradataEl {
            database: self.database,
            host: self.host,
            port: self.port,
        }
    }
}

pub struct QuicksightDataSourceParametersElTeradataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElTeradataElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSourceParametersElTeradataElRef {
        QuicksightDataSourceParametersElTeradataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElTeradataElRef {
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
pub struct QuicksightDataSourceParametersElTwitterEl {
    max_rows: terrars::PrimField<f64>,
    query: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSourceParametersElTwitterEl { }

impl terrars::ToListMappable for QuicksightDataSourceParametersElTwitterEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersElTwitterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersElTwitterEl {
    #[doc = ""]
    pub max_rows: terrars::PrimField<f64>,
    #[doc = ""]
    pub query: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSourceParametersElTwitterEl {
    pub fn build(self) -> QuicksightDataSourceParametersElTwitterEl {
        QuicksightDataSourceParametersElTwitterEl {
            max_rows: self.max_rows,
            query: self.query,
        }
    }
}

pub struct QuicksightDataSourceParametersElTwitterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElTwitterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSourceParametersElTwitterElRef {
        QuicksightDataSourceParametersElTwitterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElTwitterElRef {
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
struct QuicksightDataSourceParametersElDynamic {
    amazon_elasticsearch: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElAmazonElasticsearchEl>>,
    athena: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElAthenaEl>>,
    aurora: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElAuroraEl>>,
    aurora_postgresql: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElAuroraPostgresqlEl>>,
    aws_iot_analytics: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElAwsIotAnalyticsEl>>,
    jira: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElJiraEl>>,
    maria_db: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElMariaDbEl>>,
    mysql: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElMysqlEl>>,
    oracle: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElOracleEl>>,
    postgresql: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElPostgresqlEl>>,
    presto: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElPrestoEl>>,
    rds: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElRdsEl>>,
    redshift: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElRedshiftEl>>,
    s3: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElS3El>>,
    service_now: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElServiceNowEl>>,
    snowflake: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElSnowflakeEl>>,
    spark: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElSparkEl>>,
    sql_server: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElSqlServerEl>>,
    teradata: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElTeradataEl>>,
    twitter: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceParametersElTwitterEl>>,
}

#[derive(Serialize)]
pub struct QuicksightDataSourceParametersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_elasticsearch: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElAmazonElasticsearchEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    athena: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElAthenaEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aurora: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElAuroraEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aurora_postgresql: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElAuroraPostgresqlEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_iot_analytics: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElAwsIotAnalyticsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jira: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElJiraEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maria_db: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElMariaDbEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mysql: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElMysqlEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oracle: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElOracleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    postgresql: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElPostgresqlEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    presto: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElPrestoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rds: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElRdsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redshift: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElRedshiftEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElS3El>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_now: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElServiceNowEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snowflake: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElSnowflakeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spark: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElSparkEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sql_server: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElSqlServerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    teradata: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElTeradataEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    twitter: std::option::Option<std::vec::Vec<QuicksightDataSourceParametersElTwitterEl>>,
    dynamic: QuicksightDataSourceParametersElDynamic,
}

impl QuicksightDataSourceParametersEl {
    #[doc = "Set the field `amazon_elasticsearch`.\n"]
    pub fn set_amazon_elasticsearch(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElAmazonElasticsearchEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElAthenaEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElAuroraEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElAuroraPostgresqlEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElAwsIotAnalyticsEl>>,
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

    #[doc = "Set the field `jira`.\n"]
    pub fn set_jira(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElJiraEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElMariaDbEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElMysqlEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElOracleEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElPostgresqlEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElPrestoEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElRdsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElRedshiftEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElS3El>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElServiceNowEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElSnowflakeEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElSparkEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElSqlServerEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElTeradataEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<QuicksightDataSourceParametersElTwitterEl>>,
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

impl terrars::ToListMappable for QuicksightDataSourceParametersEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceParametersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceParametersEl {}

impl BuildQuicksightDataSourceParametersEl {
    pub fn build(self) -> QuicksightDataSourceParametersEl {
        QuicksightDataSourceParametersEl {
            amazon_elasticsearch: core::default::Default::default(),
            athena: core::default::Default::default(),
            aurora: core::default::Default::default(),
            aurora_postgresql: core::default::Default::default(),
            aws_iot_analytics: core::default::Default::default(),
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

pub struct QuicksightDataSourceParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceParametersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSourceParametersElRef {
        QuicksightDataSourceParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amazon_elasticsearch` after provisioning.\n"]
    pub fn amazon_elasticsearch(&self) -> terrars::ListRef<QuicksightDataSourceParametersElAmazonElasticsearchElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amazon_elasticsearch", self.base))
    }

    #[doc = "Get a reference to the value of field `athena` after provisioning.\n"]
    pub fn athena(&self) -> terrars::ListRef<QuicksightDataSourceParametersElAthenaElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.athena", self.base))
    }

    #[doc = "Get a reference to the value of field `aurora` after provisioning.\n"]
    pub fn aurora(&self) -> terrars::ListRef<QuicksightDataSourceParametersElAuroraElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aurora", self.base))
    }

    #[doc = "Get a reference to the value of field `aurora_postgresql` after provisioning.\n"]
    pub fn aurora_postgresql(&self) -> terrars::ListRef<QuicksightDataSourceParametersElAuroraPostgresqlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aurora_postgresql", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_iot_analytics` after provisioning.\n"]
    pub fn aws_iot_analytics(&self) -> terrars::ListRef<QuicksightDataSourceParametersElAwsIotAnalyticsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_iot_analytics", self.base))
    }

    #[doc = "Get a reference to the value of field `jira` after provisioning.\n"]
    pub fn jira(&self) -> terrars::ListRef<QuicksightDataSourceParametersElJiraElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jira", self.base))
    }

    #[doc = "Get a reference to the value of field `maria_db` after provisioning.\n"]
    pub fn maria_db(&self) -> terrars::ListRef<QuicksightDataSourceParametersElMariaDbElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maria_db", self.base))
    }

    #[doc = "Get a reference to the value of field `mysql` after provisioning.\n"]
    pub fn mysql(&self) -> terrars::ListRef<QuicksightDataSourceParametersElMysqlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mysql", self.base))
    }

    #[doc = "Get a reference to the value of field `oracle` after provisioning.\n"]
    pub fn oracle(&self) -> terrars::ListRef<QuicksightDataSourceParametersElOracleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oracle", self.base))
    }

    #[doc = "Get a reference to the value of field `postgresql` after provisioning.\n"]
    pub fn postgresql(&self) -> terrars::ListRef<QuicksightDataSourceParametersElPostgresqlElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.postgresql", self.base))
    }

    #[doc = "Get a reference to the value of field `presto` after provisioning.\n"]
    pub fn presto(&self) -> terrars::ListRef<QuicksightDataSourceParametersElPrestoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.presto", self.base))
    }

    #[doc = "Get a reference to the value of field `rds` after provisioning.\n"]
    pub fn rds(&self) -> terrars::ListRef<QuicksightDataSourceParametersElRdsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rds", self.base))
    }

    #[doc = "Get a reference to the value of field `redshift` after provisioning.\n"]
    pub fn redshift(&self) -> terrars::ListRef<QuicksightDataSourceParametersElRedshiftElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redshift", self.base))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<QuicksightDataSourceParametersElS3ElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }

    #[doc = "Get a reference to the value of field `service_now` after provisioning.\n"]
    pub fn service_now(&self) -> terrars::ListRef<QuicksightDataSourceParametersElServiceNowElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_now", self.base))
    }

    #[doc = "Get a reference to the value of field `snowflake` after provisioning.\n"]
    pub fn snowflake(&self) -> terrars::ListRef<QuicksightDataSourceParametersElSnowflakeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.snowflake", self.base))
    }

    #[doc = "Get a reference to the value of field `spark` after provisioning.\n"]
    pub fn spark(&self) -> terrars::ListRef<QuicksightDataSourceParametersElSparkElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spark", self.base))
    }

    #[doc = "Get a reference to the value of field `sql_server` after provisioning.\n"]
    pub fn sql_server(&self) -> terrars::ListRef<QuicksightDataSourceParametersElSqlServerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sql_server", self.base))
    }

    #[doc = "Get a reference to the value of field `teradata` after provisioning.\n"]
    pub fn teradata(&self) -> terrars::ListRef<QuicksightDataSourceParametersElTeradataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.teradata", self.base))
    }

    #[doc = "Get a reference to the value of field `twitter` after provisioning.\n"]
    pub fn twitter(&self) -> terrars::ListRef<QuicksightDataSourceParametersElTwitterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.twitter", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSourcePermissionEl {
    actions: terrars::SetField<terrars::PrimField<std::string::String>>,
    principal: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSourcePermissionEl { }

impl terrars::ToListMappable for QuicksightDataSourcePermissionEl {
    type O = terrars::BlockAssignable<QuicksightDataSourcePermissionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourcePermissionEl {
    #[doc = ""]
    pub actions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub principal: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSourcePermissionEl {
    pub fn build(self) -> QuicksightDataSourcePermissionEl {
        QuicksightDataSourcePermissionEl {
            actions: self.actions,
            principal: self.principal,
        }
    }
}

pub struct QuicksightDataSourcePermissionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourcePermissionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSourcePermissionElRef {
        QuicksightDataSourcePermissionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourcePermissionElRef {
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
pub struct QuicksightDataSourceSslPropertiesEl {
    disable_ssl: terrars::PrimField<bool>,
}

impl QuicksightDataSourceSslPropertiesEl { }

impl terrars::ToListMappable for QuicksightDataSourceSslPropertiesEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceSslPropertiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceSslPropertiesEl {
    #[doc = ""]
    pub disable_ssl: terrars::PrimField<bool>,
}

impl BuildQuicksightDataSourceSslPropertiesEl {
    pub fn build(self) -> QuicksightDataSourceSslPropertiesEl {
        QuicksightDataSourceSslPropertiesEl { disable_ssl: self.disable_ssl }
    }
}

pub struct QuicksightDataSourceSslPropertiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceSslPropertiesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> QuicksightDataSourceSslPropertiesElRef {
        QuicksightDataSourceSslPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceSslPropertiesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disable_ssl` after provisioning.\n"]
    pub fn disable_ssl(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_ssl", self.base))
    }
}

#[derive(Serialize)]
pub struct QuicksightDataSourceVpcConnectionPropertiesEl {
    vpc_connection_arn: terrars::PrimField<std::string::String>,
}

impl QuicksightDataSourceVpcConnectionPropertiesEl { }

impl terrars::ToListMappable for QuicksightDataSourceVpcConnectionPropertiesEl {
    type O = terrars::BlockAssignable<QuicksightDataSourceVpcConnectionPropertiesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildQuicksightDataSourceVpcConnectionPropertiesEl {
    #[doc = ""]
    pub vpc_connection_arn: terrars::PrimField<std::string::String>,
}

impl BuildQuicksightDataSourceVpcConnectionPropertiesEl {
    pub fn build(self) -> QuicksightDataSourceVpcConnectionPropertiesEl {
        QuicksightDataSourceVpcConnectionPropertiesEl { vpc_connection_arn: self.vpc_connection_arn }
    }
}

pub struct QuicksightDataSourceVpcConnectionPropertiesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for QuicksightDataSourceVpcConnectionPropertiesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> QuicksightDataSourceVpcConnectionPropertiesElRef {
        QuicksightDataSourceVpcConnectionPropertiesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl QuicksightDataSourceVpcConnectionPropertiesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vpc_connection_arn` after provisioning.\n"]
    pub fn vpc_connection_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_connection_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct QuicksightDataSourceDynamic {
    credentials: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceCredentialsEl>>,
    parameters: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceParametersEl>>,
    permission: std::option::Option<terrars::DynamicBlock<QuicksightDataSourcePermissionEl>>,
    ssl_properties: std::option::Option<terrars::DynamicBlock<QuicksightDataSourceSslPropertiesEl>>,
    vpc_connection_properties: std
    ::option
    ::Option<terrars::DynamicBlock<QuicksightDataSourceVpcConnectionPropertiesEl>>,
}
