#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GlueConnectionData_ {
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
    athena_properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    catalog_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_criteria: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication_configuration: std::option::Option<std::vec::Vec<GlueConnection_AuthenticationConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    physical_connection_requirements: std::option::Option<std::vec::Vec<GlueConnection_PhysicalConnectionRequirements>>,
    dynamic: GlueConnectionDynamic,
}

struct GlueConnection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GlueConnectionData_>,
}

#[derive(Clone)]
pub struct GlueConnection(std::rc::Rc<GlueConnection_>);

impl GlueConnection {
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

    #[doc = "Set the field `athena_properties`.\n"]
    pub fn set_athena_properties(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().athena_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `catalog_id`.\n"]
    pub fn set_catalog_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().catalog_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_properties`.\n"]
    pub fn set_connection_properties(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().connection_properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_type`.\n"]
    pub fn set_connection_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().connection_type = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `match_criteria`.\n"]
    pub fn set_match_criteria(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().match_criteria = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `authentication_configuration`.\n"]
    pub fn set_authentication_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueConnection_AuthenticationConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().authentication_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.authentication_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `physical_connection_requirements`.\n"]
    pub fn set_physical_connection_requirements(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GlueConnection_PhysicalConnectionRequirements>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().physical_connection_requirements = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.physical_connection_requirements = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `athena_properties` after provisioning.\n"]
    pub fn athena_properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.athena_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_properties` after provisioning.\n"]
    pub fn connection_properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.connection_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `match_criteria` after provisioning.\n"]
    pub fn match_criteria(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_criteria", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `authentication_configuration` after provisioning.\n"]
    pub fn authentication_configuration(&self) -> terrars::ListRef<GlueConnection_AuthenticationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `physical_connection_requirements` after provisioning.\n"]
    pub fn physical_connection_requirements(&self) -> terrars::ListRef<GlueConnection_PhysicalConnectionRequirementsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.physical_connection_requirements", self.extract_ref()),
        )
    }
}

impl terrars::Referable for GlueConnection {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GlueConnection { }

impl terrars::ToListMappable for GlueConnection {
    type O = terrars::ListRef<GlueConnectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GlueConnection_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_glue_connection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGlueConnection {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGlueConnection {
    pub fn build(self, stack: &mut terrars::Stack) -> GlueConnection {
        let out = GlueConnection(std::rc::Rc::new(GlueConnection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GlueConnectionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                athena_properties: core::default::Default::default(),
                catalog_id: core::default::Default::default(),
                connection_properties: core::default::Default::default(),
                connection_type: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                match_criteria: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                authentication_configuration: core::default::Default::default(),
                physical_connection_requirements: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GlueConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueConnectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GlueConnectionRef {
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

    #[doc = "Get a reference to the value of field `athena_properties` after provisioning.\n"]
    pub fn athena_properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.athena_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `catalog_id` after provisioning.\n"]
    pub fn catalog_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.catalog_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_properties` after provisioning.\n"]
    pub fn connection_properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.connection_properties", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_type` after provisioning.\n"]
    pub fn connection_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `match_criteria` after provisioning.\n"]
    pub fn match_criteria(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_criteria", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `authentication_configuration` after provisioning.\n"]
    pub fn authentication_configuration(&self) -> terrars::ListRef<GlueConnection_AuthenticationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authentication_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `physical_connection_requirements` after provisioning.\n"]
    pub fn physical_connection_requirements(&self) -> terrars::ListRef<GlueConnection_PhysicalConnectionRequirementsRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.physical_connection_requirements", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct GlueConnectionDynamic {
    authentication_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<GlueConnection_AuthenticationConfiguration>>,
    physical_connection_requirements: std
    ::option
    ::Option<terrars::DynamicBlock<GlueConnection_PhysicalConnectionRequirements>>,
}

#[derive(Serialize)]
pub struct GlueConnection_AuthenticationConfiguration_BasicAuthenticationCredentials {
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl GlueConnection_AuthenticationConfiguration_BasicAuthenticationCredentials { }

impl terrars::ToListMappable for GlueConnection_AuthenticationConfiguration_BasicAuthenticationCredentials {
    type O = terrars::BlockAssignable<GlueConnection_AuthenticationConfiguration_BasicAuthenticationCredentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueConnection_AuthenticationConfiguration_BasicAuthenticationCredentials {
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildGlueConnection_AuthenticationConfiguration_BasicAuthenticationCredentials {
    pub fn build(self) -> GlueConnection_AuthenticationConfiguration_BasicAuthenticationCredentials {
        GlueConnection_AuthenticationConfiguration_BasicAuthenticationCredentials {
            password: self.password,
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct GlueConnection_AuthenticationConfiguration_BasicAuthenticationCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueConnection_AuthenticationConfiguration_BasicAuthenticationCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueConnection_AuthenticationConfiguration_BasicAuthenticationCredentialsRef {
        GlueConnection_AuthenticationConfiguration_BasicAuthenticationCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueConnection_AuthenticationConfiguration_BasicAuthenticationCredentialsRef {
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
pub struct GlueConnection_AuthenticationConfiguration_Oauth2Properties_AuthorizationCodeProperties {
    authorization_code: terrars::PrimField<std::string::String>,
    redirect_uri: terrars::PrimField<std::string::String>,
}

impl GlueConnection_AuthenticationConfiguration_Oauth2Properties_AuthorizationCodeProperties { }

impl terrars
::ToListMappable for GlueConnection_AuthenticationConfiguration_Oauth2Properties_AuthorizationCodeProperties {
    type O =
        terrars
        ::BlockAssignable<GlueConnection_AuthenticationConfiguration_Oauth2Properties_AuthorizationCodeProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueConnection_AuthenticationConfiguration_Oauth2Properties_AuthorizationCodeProperties {
    #[doc = ""]
    pub authorization_code: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub redirect_uri: terrars::PrimField<std::string::String>,
}

impl BuildGlueConnection_AuthenticationConfiguration_Oauth2Properties_AuthorizationCodeProperties {
    pub fn build(self) -> GlueConnection_AuthenticationConfiguration_Oauth2Properties_AuthorizationCodeProperties {
        GlueConnection_AuthenticationConfiguration_Oauth2Properties_AuthorizationCodeProperties {
            authorization_code: self.authorization_code,
            redirect_uri: self.redirect_uri,
        }
    }
}

#[allow(dead_code)]
pub struct GlueConnection_AuthenticationConfiguration_Oauth2Properties_AuthorizationCodePropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueConnection_AuthenticationConfiguration_Oauth2Properties_AuthorizationCodePropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueConnection_AuthenticationConfiguration_Oauth2Properties_AuthorizationCodePropertiesRef {
        GlueConnection_AuthenticationConfiguration_Oauth2Properties_AuthorizationCodePropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueConnection_AuthenticationConfiguration_Oauth2Properties_AuthorizationCodePropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_code` after provisioning.\n"]
    pub fn authorization_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_code", self.base))
    }

    #[doc = "Get a reference to the value of field `redirect_uri` after provisioning.\n"]
    pub fn redirect_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.redirect_uri", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2ClientApplication {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_managed_client_application_reference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_managed_client_application_client_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2ClientApplication {
    #[doc = "Set the field `aws_managed_client_application_reference`.\n"]
    pub fn set_aws_managed_client_application_reference(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.aws_managed_client_application_reference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_managed_client_application_client_id`.\n"]
    pub fn set_user_managed_client_application_client_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.user_managed_client_application_client_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2ClientApplication {
    type O =
        terrars
        ::BlockAssignable<GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2ClientApplication>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2ClientApplication {}

impl BuildGlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2ClientApplication {
    pub fn build(self) -> GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2ClientApplication {
        GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2ClientApplication {
            aws_managed_client_application_reference: core::default::Default::default(),
            user_managed_client_application_client_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2ClientApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2ClientApplicationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2ClientApplicationRef {
        GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2ClientApplicationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2ClientApplicationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_managed_client_application_reference` after provisioning.\n"]
    pub fn aws_managed_client_application_reference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.aws_managed_client_application_reference", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `user_managed_client_application_client_id` after provisioning.\n"]
    pub fn user_managed_client_application_client_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_managed_client_application_client_id", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2Credentials {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jwt_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    refresh_token: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_managed_client_application_client_secret: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2Credentials {
    #[doc = "Set the field `access_token`.\n"]
    pub fn set_access_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `jwt_token`.\n"]
    pub fn set_jwt_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.jwt_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `refresh_token`.\n"]
    pub fn set_refresh_token(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.refresh_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_managed_client_application_client_secret`.\n"]
    pub fn set_user_managed_client_application_client_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.user_managed_client_application_client_secret = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2Credentials {
    type O = terrars::BlockAssignable<GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2Credentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2Credentials {}

impl BuildGlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2Credentials {
    pub fn build(self) -> GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2Credentials {
        GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2Credentials {
            access_token: core::default::Default::default(),
            jwt_token: core::default::Default::default(),
            refresh_token: core::default::Default::default(),
            user_managed_client_application_client_secret: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2CredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2CredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2CredentialsRef {
        GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2CredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2CredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_token` after provisioning.\n"]
    pub fn access_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_token", self.base))
    }

    #[doc = "Get a reference to the value of field `jwt_token` after provisioning.\n"]
    pub fn jwt_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jwt_token", self.base))
    }

    #[doc = "Get a reference to the value of field `refresh_token` after provisioning.\n"]
    pub fn refresh_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.refresh_token", self.base))
    }

    #[doc =
        "Get a reference to the value of field `user_managed_client_application_client_secret` after provisioning.\n"]
    pub fn user_managed_client_application_client_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.user_managed_client_application_client_secret", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct GlueConnection_AuthenticationConfiguration_Oauth2PropertiesDynamic {
    authorization_code_properties: std
    ::option
    ::Option<
        terrars::DynamicBlock<GlueConnection_AuthenticationConfiguration_Oauth2Properties_AuthorizationCodeProperties>,
    >,
    oauth2_client_application: std
    ::option
    ::Option<
        terrars::DynamicBlock<GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2ClientApplication>,
    >,
    oauth2_credentials: std
    ::option
    ::Option<terrars::DynamicBlock<GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2Credentials>>,
}

#[derive(Serialize)]
pub struct GlueConnection_AuthenticationConfiguration_Oauth2Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_grant_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_url_parameters_map: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_code_properties: std
    ::option
    ::Option<std::vec::Vec<GlueConnection_AuthenticationConfiguration_Oauth2Properties_AuthorizationCodeProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_client_application: std
    ::option
    ::Option<std::vec::Vec<GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2ClientApplication>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_credentials: std
    ::option
    ::Option<std::vec::Vec<GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2Credentials>>,
    dynamic: GlueConnection_AuthenticationConfiguration_Oauth2PropertiesDynamic,
}

impl GlueConnection_AuthenticationConfiguration_Oauth2Properties {
    #[doc = "Set the field `oauth2_grant_type`.\n"]
    pub fn set_oauth2_grant_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.oauth2_grant_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_url`.\n"]
    pub fn set_token_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.token_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `token_url_parameters_map`.\n"]
    pub fn set_token_url_parameters_map(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.token_url_parameters_map = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorization_code_properties`.\n"]
    pub fn set_authorization_code_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GlueConnection_AuthenticationConfiguration_Oauth2Properties_AuthorizationCodeProperties,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authorization_code_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authorization_code_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth2_client_application`.\n"]
    pub fn set_oauth2_client_application(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2ClientApplication,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth2_client_application = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2_client_application = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth2_credentials`.\n"]
    pub fn set_oauth2_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2Credentials,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth2_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2_credentials = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueConnection_AuthenticationConfiguration_Oauth2Properties {
    type O = terrars::BlockAssignable<GlueConnection_AuthenticationConfiguration_Oauth2Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueConnection_AuthenticationConfiguration_Oauth2Properties {}

impl BuildGlueConnection_AuthenticationConfiguration_Oauth2Properties {
    pub fn build(self) -> GlueConnection_AuthenticationConfiguration_Oauth2Properties {
        GlueConnection_AuthenticationConfiguration_Oauth2Properties {
            oauth2_grant_type: core::default::Default::default(),
            token_url: core::default::Default::default(),
            token_url_parameters_map: core::default::Default::default(),
            authorization_code_properties: core::default::Default::default(),
            oauth2_client_application: core::default::Default::default(),
            oauth2_credentials: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueConnection_AuthenticationConfiguration_Oauth2PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueConnection_AuthenticationConfiguration_Oauth2PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueConnection_AuthenticationConfiguration_Oauth2PropertiesRef {
        GlueConnection_AuthenticationConfiguration_Oauth2PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueConnection_AuthenticationConfiguration_Oauth2PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `oauth2_grant_type` after provisioning.\n"]
    pub fn oauth2_grant_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oauth2_grant_type", self.base))
    }

    #[doc = "Get a reference to the value of field `token_url` after provisioning.\n"]
    pub fn token_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.token_url", self.base))
    }

    #[doc = "Get a reference to the value of field `token_url_parameters_map` after provisioning.\n"]
    pub fn token_url_parameters_map(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.token_url_parameters_map", self.base))
    }

    #[doc = "Get a reference to the value of field `authorization_code_properties` after provisioning.\n"]
    pub fn authorization_code_properties(
        &self,
    ) -> terrars::ListRef<GlueConnection_AuthenticationConfiguration_Oauth2Properties_AuthorizationCodePropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization_code_properties", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_client_application` after provisioning.\n"]
    pub fn oauth2_client_application(
        &self,
    ) -> terrars::ListRef<GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2ClientApplicationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2_client_application", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_credentials` after provisioning.\n"]
    pub fn oauth2_credentials(
        &self,
    ) -> terrars::ListRef<GlueConnection_AuthenticationConfiguration_Oauth2Properties_Oauth2CredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2_credentials", self.base))
    }
}

#[derive(Serialize, Default)]
struct GlueConnection_AuthenticationConfigurationDynamic {
    basic_authentication_credentials: std
    ::option
    ::Option<terrars::DynamicBlock<GlueConnection_AuthenticationConfiguration_BasicAuthenticationCredentials>>,
    oauth2_properties: std
    ::option
    ::Option<terrars::DynamicBlock<GlueConnection_AuthenticationConfiguration_Oauth2Properties>>,
}

#[derive(Serialize)]
pub struct GlueConnection_AuthenticationConfiguration {
    authentication_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_authentication_credentials: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_authentication_credentials: std
    ::option
    ::Option<std::vec::Vec<GlueConnection_AuthenticationConfiguration_BasicAuthenticationCredentials>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oauth2_properties: std::option::Option<std::vec::Vec<GlueConnection_AuthenticationConfiguration_Oauth2Properties>>,
    dynamic: GlueConnection_AuthenticationConfigurationDynamic,
}

impl GlueConnection_AuthenticationConfiguration {
    #[doc = "Set the field `custom_authentication_credentials`.\n"]
    pub fn set_custom_authentication_credentials(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.custom_authentication_credentials = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secret_arn`.\n"]
    pub fn set_secret_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `basic_authentication_credentials`.\n"]
    pub fn set_basic_authentication_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GlueConnection_AuthenticationConfiguration_BasicAuthenticationCredentials>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.basic_authentication_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.basic_authentication_credentials = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `oauth2_properties`.\n"]
    pub fn set_oauth2_properties(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GlueConnection_AuthenticationConfiguration_Oauth2Properties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.oauth2_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.oauth2_properties = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GlueConnection_AuthenticationConfiguration {
    type O = terrars::BlockAssignable<GlueConnection_AuthenticationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueConnection_AuthenticationConfiguration {
    #[doc = ""]
    pub authentication_type: terrars::PrimField<std::string::String>,
}

impl BuildGlueConnection_AuthenticationConfiguration {
    pub fn build(self) -> GlueConnection_AuthenticationConfiguration {
        GlueConnection_AuthenticationConfiguration {
            authentication_type: self.authentication_type,
            custom_authentication_credentials: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
            secret_arn: core::default::Default::default(),
            basic_authentication_credentials: core::default::Default::default(),
            oauth2_properties: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueConnection_AuthenticationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueConnection_AuthenticationConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GlueConnection_AuthenticationConfigurationRef {
        GlueConnection_AuthenticationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueConnection_AuthenticationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authentication_type` after provisioning.\n"]
    pub fn authentication_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication_type", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_authentication_credentials` after provisioning.\n"]
    pub fn custom_authentication_credentials(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.custom_authentication_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `basic_authentication_credentials` after provisioning.\n"]
    pub fn basic_authentication_credentials(
        &self,
    ) -> terrars::ListRef<GlueConnection_AuthenticationConfiguration_BasicAuthenticationCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.basic_authentication_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `oauth2_properties` after provisioning.\n"]
    pub fn oauth2_properties(&self) -> terrars::ListRef<GlueConnection_AuthenticationConfiguration_Oauth2PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.oauth2_properties", self.base))
    }
}

#[derive(Serialize)]
pub struct GlueConnection_PhysicalConnectionRequirements {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_id_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GlueConnection_PhysicalConnectionRequirements {
    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.availability_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_id_list`.\n"]
    pub fn set_security_group_id_list(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_id_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GlueConnection_PhysicalConnectionRequirements {
    type O = terrars::BlockAssignable<GlueConnection_PhysicalConnectionRequirements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGlueConnection_PhysicalConnectionRequirements {}

impl BuildGlueConnection_PhysicalConnectionRequirements {
    pub fn build(self) -> GlueConnection_PhysicalConnectionRequirements {
        GlueConnection_PhysicalConnectionRequirements {
            availability_zone: core::default::Default::default(),
            security_group_id_list: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GlueConnection_PhysicalConnectionRequirementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GlueConnection_PhysicalConnectionRequirementsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GlueConnection_PhysicalConnectionRequirementsRef {
        GlueConnection_PhysicalConnectionRequirementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GlueConnection_PhysicalConnectionRequirementsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_id_list` after provisioning.\n"]
    pub fn security_group_id_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_id_list", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}
