#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentcoreRegistryData_ {
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
    approval_configuration: std::option::Option<terrars::ListField<BedrockagentcoreRegistry_ApprovalConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizer_configuration: std::option::Option<std::vec::Vec<BedrockagentcoreRegistry_AuthorizerConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockagentcoreRegistry_Timeouts>,
    dynamic: BedrockagentcoreRegistryDynamic,
}

struct BedrockagentcoreRegistry_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentcoreRegistryData_>,
}

#[derive(Clone)]
pub struct BedrockagentcoreRegistry(std::rc::Rc<BedrockagentcoreRegistry_>);

impl BedrockagentcoreRegistry {
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

    #[doc = "Set the field `approval_configuration`.\n"]
    pub fn set_approval_configuration(
        self,
        v: impl std::convert::Into<terrars::ListField<BedrockagentcoreRegistry_ApprovalConfiguration>>,
    ) -> Self {
        self.0.data.borrow_mut().approval_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorizer_type`.\n"]
    pub fn set_authorizer_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().authorizer_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorizer_configuration`.\n"]
    pub fn set_authorizer_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreRegistry_AuthorizerConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().authorizer_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.authorizer_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockagentcoreRegistry_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `approval_configuration` after provisioning.\n"]
    pub fn approval_configuration(&self) -> terrars::ListRef<BedrockagentcoreRegistry_ApprovalConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_type` after provisioning.\n"]
    pub fn authorizer_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorizer_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `registry_arn` after provisioning.\n"]
    pub fn registry_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_configuration` after provisioning.\n"]
    pub fn authorizer_configuration(&self) -> terrars::ListRef<BedrockagentcoreRegistry_AuthorizerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreRegistry_TimeoutsRef {
        BedrockagentcoreRegistry_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BedrockagentcoreRegistry {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentcoreRegistry { }

impl terrars::ToListMappable for BedrockagentcoreRegistry {
    type O = terrars::ListRef<BedrockagentcoreRegistryRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentcoreRegistry_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagentcore_registry".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentcoreRegistry {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreRegistry {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentcoreRegistry {
        let out = BedrockagentcoreRegistry(std::rc::Rc::new(BedrockagentcoreRegistry_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockagentcoreRegistryData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                approval_configuration: core::default::Default::default(),
                authorizer_type: core::default::Default::default(),
                description: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                authorizer_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentcoreRegistryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreRegistryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentcoreRegistryRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `approval_configuration` after provisioning.\n"]
    pub fn approval_configuration(&self) -> terrars::ListRef<BedrockagentcoreRegistry_ApprovalConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_type` after provisioning.\n"]
    pub fn authorizer_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorizer_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `registry_arn` after provisioning.\n"]
    pub fn registry_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authorizer_configuration` after provisioning.\n"]
    pub fn authorizer_configuration(&self) -> terrars::ListRef<BedrockagentcoreRegistry_AuthorizerConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizer_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreRegistry_TimeoutsRef {
        BedrockagentcoreRegistry_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreRegistryDynamic {
    authorizer_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreRegistry_AuthorizerConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreRegistry_ApprovalConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_approval: std::option::Option<terrars::PrimField<bool>>,
}

impl BedrockagentcoreRegistry_ApprovalConfiguration {
    #[doc = "Set the field `auto_approval`.\n"]
    pub fn set_auto_approval(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.auto_approval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreRegistry_ApprovalConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreRegistry_ApprovalConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreRegistry_ApprovalConfiguration {}

impl BuildBedrockagentcoreRegistry_ApprovalConfiguration {
    pub fn build(self) -> BedrockagentcoreRegistry_ApprovalConfiguration {
        BedrockagentcoreRegistry_ApprovalConfiguration { auto_approval: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreRegistry_ApprovalConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreRegistry_ApprovalConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreRegistry_ApprovalConfigurationRef {
        BedrockagentcoreRegistry_ApprovalConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreRegistry_ApprovalConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_approval` after provisioning.\n"]
    pub fn auto_approval(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_approval", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
    arn: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {

}

impl terrars
::ToListMappable for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
    pub fn build(
        self,
    ) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment {
            arn: self.arn,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationDynamic {
    hosting_environment: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workload_identities: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosting_environment: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment,
        >,
    >,
    dynamic: BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationDynamic,
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
    #[doc = "Set the field `workload_identities`.\n"]
    pub fn set_workload_identities(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.workload_identities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hosting_environment`.\n"]
    pub fn set_hosting_environment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironment,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.hosting_environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.hosting_environment = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {}

impl BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
    pub fn build(
        self,
    ) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration {
            workload_identities: core::default::Default::default(),
            hosting_environment: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `workload_identities` after provisioning.\n"]
    pub fn workload_identities(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workload_identities", self.base))
    }

    #[doc = "Get a reference to the value of field `hosting_environment` after provisioning.\n"]
    pub fn hosting_environment(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration_HostingEnvironmentRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hosting_environment", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_value_string: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_value_string_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
    #[doc = "Set the field `match_value_string`.\n"]
    pub fn set_match_value_string(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.match_value_string = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_value_string_list`.\n"]
    pub fn set_match_value_string_list(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.match_value_string_list = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {}

impl BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
    pub fn build(
        self,
    ) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue {
            match_value_string: core::default::Default::default(),
            match_value_string_list: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_value_string` after provisioning.\n"]
    pub fn match_value_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match_value_string", self.base))
    }

    #[doc = "Get a reference to the value of field `match_value_string_list` after provisioning.\n"]
    pub fn match_value_string_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.match_value_string_list", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueDynamic {
    claim_match_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    claim_match_operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    claim_match_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue,
        >,
    >,
    dynamic: BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueDynamic,
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    #[doc = "Set the field `claim_match_value`.\n"]
    pub fn set_claim_match_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.claim_match_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.claim_match_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    #[doc = ""]
    pub claim_match_operator: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
    pub fn build(
        self,
    ) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue {
            claim_match_operator: self.claim_match_operator,
            claim_match_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `claim_match_operator` after provisioning.\n"]
    pub fn claim_match_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.claim_match_operator", self.base))
    }

    #[doc = "Get a reference to the value of field `claim_match_value` after provisioning.\n"]
    pub fn claim_match_value(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue_ClaimMatchValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.claim_match_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimDynamic {
    authorizing_claim_match_value: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    inbound_token_claim_name: terrars::PrimField<std::string::String>,
    inbound_token_claim_value_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorizing_claim_match_value: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue,
        >,
    >,
    dynamic: BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimDynamic,
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    #[doc = "Set the field `authorizing_claim_match_value`.\n"]
    pub fn set_authorizing_claim_match_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValue,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authorizing_claim_match_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authorizing_claim_match_value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    type O =
        terrars::BlockAssignable<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    #[doc = ""]
    pub inbound_token_claim_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub inbound_token_claim_value_type: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
    pub fn build(self) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim {
            inbound_token_claim_name: self.inbound_token_claim_name,
            inbound_token_claim_value_type: self.inbound_token_claim_value_type,
            authorizing_claim_match_value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaimRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `inbound_token_claim_name` after provisioning.\n"]
    pub fn inbound_token_claim_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inbound_token_claim_name", self.base))
    }

    #[doc = "Get a reference to the value of field `inbound_token_claim_value_type` after provisioning.\n"]
    pub fn inbound_token_claim_value_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inbound_token_claim_value_type", self.base))
    }

    #[doc = "Get a reference to the value of field `authorizing_claim_match_value` after provisioning.\n"]
    pub fn authorizing_claim_match_value(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim_AuthorizingClaimMatchValueRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorizing_claim_match_value", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    endpoint_ip_address_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    routing_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    vpc_identifier: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    #[doc = "Set the field `routing_domain`.\n"]
    pub fn set_routing_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.routing_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    #[doc = ""]
    pub endpoint_ip_address_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub vpc_identifier: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
    pub fn build(
        self,
    ) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource {
            endpoint_ip_address_type: self.endpoint_ip_address_type,
            routing_domain: core::default::Default::default(),
            security_group_ids: core::default::Default::default(),
            subnet_ids: self.subnet_ids,
            tags: core::default::Default::default(),
            vpc_identifier: self.vpc_identifier,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_ip_address_type` after provisioning.\n"]
    pub fn endpoint_ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_ip_address_type", self.base))
    }

    #[doc = "Get a reference to the value of field `routing_domain` after provisioning.\n"]
    pub fn routing_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_domain", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_identifier` after provisioning.\n"]
    pub fn vpc_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_configuration_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
    #[doc = "Set the field `resource_configuration_identifier`.\n"]
    pub fn set_resource_configuration_identifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.resource_configuration_identifier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {}

impl BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
    pub fn build(
        self,
    ) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource {
            resource_configuration_identifier: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_configuration_identifier` after provisioning.\n"]
    pub fn resource_configuration_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_configuration_identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointDynamic {
    managed_vpc_resource: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource,
        >,
    >,
    self_managed_lattice_resource: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_vpc_resource: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed_lattice_resource: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource,
        >,
    >,
    dynamic: BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointDynamic,
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
    #[doc = "Set the field `managed_vpc_resource`.\n"]
    pub fn set_managed_vpc_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_vpc_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_vpc_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `self_managed_lattice_resource`.\n"]
    pub fn set_self_managed_lattice_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResource,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.self_managed_lattice_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.self_managed_lattice_resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {}

impl BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
    pub fn build(self) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint {
            managed_vpc_resource: core::default::Default::default(),
            self_managed_lattice_resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `managed_vpc_resource` after provisioning.\n"]
    pub fn managed_vpc_resource(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_ManagedVpcResourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_vpc_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `self_managed_lattice_resource` after provisioning.\n"]
    pub fn self_managed_lattice_resource(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint_SelfManagedLatticeResourceRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.self_managed_lattice_resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesDynamic {
    private_endpoint: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
    domain: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>>,
    dynamic: BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesDynamic,
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
    #[doc = "Set the field `private_endpoint`.\n"]
    pub fn set_private_endpoint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_endpoint = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
    #[doc = ""]
    pub domain: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
    pub fn build(
        self,
    ) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides {
            domain: self.domain,
            private_endpoint: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoint` after provisioning.\n"]
    pub fn private_endpoint(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizerDynamic {
    allowed_workload_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration,
        >,
    >,
    custom_claim: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim>>,
    private_endpoint: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>,
    >,
    private_endpoint_overrides: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_audience: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_clients: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_scopes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    discovery_url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_workload_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_claim: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_endpoint_overrides: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides>,
    >,
    dynamic: BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizerDynamic,
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer {
    #[doc = "Set the field `allowed_audience`.\n"]
    pub fn set_allowed_audience(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_audience = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_clients`.\n"]
    pub fn set_allowed_clients(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_clients = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_scopes`.\n"]
    pub fn set_allowed_scopes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_scopes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_workload_configuration`.\n"]
    pub fn set_allowed_workload_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allowed_workload_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allowed_workload_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_claim`.\n"]
    pub fn set_custom_claim(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_CustomClaim,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_claim = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_claim = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_endpoint`.\n"]
    pub fn set_private_endpoint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpoint,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_endpoint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_endpoint_overrides`.\n"]
    pub fn set_private_endpoint_overrides(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverrides,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_endpoint_overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_endpoint_overrides = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer {
    type O = terrars::BlockAssignable<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer {
    #[doc = ""]
    pub discovery_url: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer {
    pub fn build(self) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer {
            allowed_audience: core::default::Default::default(),
            allowed_clients: core::default::Default::default(),
            allowed_scopes: core::default::Default::default(),
            discovery_url: self.discovery_url,
            allowed_workload_configuration: core::default::Default::default(),
            custom_claim: core::default::Default::default(),
            private_endpoint: core::default::Default::default(),
            private_endpoint_overrides: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizerRef {
        BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_audience` after provisioning.\n"]
    pub fn allowed_audience(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_audience", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_clients` after provisioning.\n"]
    pub fn allowed_clients(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_clients", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_scopes` after provisioning.\n"]
    pub fn allowed_scopes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_scopes", self.base))
    }

    #[doc = "Get a reference to the value of field `discovery_url` after provisioning.\n"]
    pub fn discovery_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.discovery_url", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_workload_configuration` after provisioning.\n"]
    pub fn allowed_workload_configuration(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_AllowedWorkloadConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_workload_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoint` after provisioning.\n"]
    pub fn private_endpoint(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `private_endpoint_overrides` after provisioning.\n"]
    pub fn private_endpoint_overrides(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer_PrivateEndpointOverridesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_endpoint_overrides", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreRegistry_AuthorizerConfigurationDynamic {
    custom_jwt_authorizer: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreRegistry_AuthorizerConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_jwt_authorizer: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer>>,
    dynamic: BedrockagentcoreRegistry_AuthorizerConfigurationDynamic,
}

impl BedrockagentcoreRegistry_AuthorizerConfiguration {
    #[doc = "Set the field `custom_jwt_authorizer`.\n"]
    pub fn set_custom_jwt_authorizer(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizer>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_jwt_authorizer = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_jwt_authorizer = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreRegistry_AuthorizerConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreRegistry_AuthorizerConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreRegistry_AuthorizerConfiguration {}

impl BuildBedrockagentcoreRegistry_AuthorizerConfiguration {
    pub fn build(self) -> BedrockagentcoreRegistry_AuthorizerConfiguration {
        BedrockagentcoreRegistry_AuthorizerConfiguration {
            custom_jwt_authorizer: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreRegistry_AuthorizerConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreRegistry_AuthorizerConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreRegistry_AuthorizerConfigurationRef {
        BedrockagentcoreRegistry_AuthorizerConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreRegistry_AuthorizerConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_jwt_authorizer` after provisioning.\n"]
    pub fn custom_jwt_authorizer(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreRegistry_AuthorizerConfiguration_CustomJwtAuthorizerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_jwt_authorizer", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreRegistry_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreRegistry_Timeouts {
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

impl terrars::ToListMappable for BedrockagentcoreRegistry_Timeouts {
    type O = terrars::BlockAssignable<BedrockagentcoreRegistry_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreRegistry_Timeouts {}

impl BuildBedrockagentcoreRegistry_Timeouts {
    pub fn build(self) -> BedrockagentcoreRegistry_Timeouts {
        BedrockagentcoreRegistry_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreRegistry_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreRegistry_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreRegistry_TimeoutsRef {
        BedrockagentcoreRegistry_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreRegistry_TimeoutsRef {
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
