#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentcoreCodeInterpreterData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<std::vec::Vec<BedrockagentcoreCodeInterpreter_Certificate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std::option::Option<std::vec::Vec<BedrockagentcoreCodeInterpreter_NetworkConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockagentcoreCodeInterpreter_Timeouts>,
    dynamic: BedrockagentcoreCodeInterpreterDynamic,
}

struct BedrockagentcoreCodeInterpreter_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentcoreCodeInterpreterData_>,
}

#[derive(Clone)]
pub struct BedrockagentcoreCodeInterpreter(std::rc::Rc<BedrockagentcoreCodeInterpreter_>);

impl BedrockagentcoreCodeInterpreter {
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

    #[doc = "Set the field `execution_role_arn`.\n"]
    pub fn set_execution_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().execution_role_arn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreCodeInterpreter_Certificate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().certificate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.certificate = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreCodeInterpreter_NetworkConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockagentcoreCodeInterpreter_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `code_interpreter_arn` after provisioning.\n"]
    pub fn code_interpreter_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_interpreter_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code_interpreter_id` after provisioning.\n"]
    pub fn code_interpreter_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_interpreter_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::ListRef<BedrockagentcoreCodeInterpreter_CertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<BedrockagentcoreCodeInterpreter_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreCodeInterpreter_TimeoutsRef {
        BedrockagentcoreCodeInterpreter_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for BedrockagentcoreCodeInterpreter {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentcoreCodeInterpreter { }

impl terrars::ToListMappable for BedrockagentcoreCodeInterpreter {
    type O = terrars::ListRef<BedrockagentcoreCodeInterpreterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentcoreCodeInterpreter_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagentcore_code_interpreter".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentcoreCodeInterpreter {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreCodeInterpreter {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentcoreCodeInterpreter {
        let out = BedrockagentcoreCodeInterpreter(std::rc::Rc::new(BedrockagentcoreCodeInterpreter_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockagentcoreCodeInterpreterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                execution_role_arn: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                certificate: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentcoreCodeInterpreterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreCodeInterpreterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentcoreCodeInterpreterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code_interpreter_arn` after provisioning.\n"]
    pub fn code_interpreter_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_interpreter_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `code_interpreter_id` after provisioning.\n"]
    pub fn code_interpreter_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code_interpreter_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::ListRef<BedrockagentcoreCodeInterpreter_CertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<BedrockagentcoreCodeInterpreter_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreCodeInterpreter_TimeoutsRef {
        BedrockagentcoreCodeInterpreter_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreCodeInterpreterDynamic {
    certificate: std::option::Option<terrars::DynamicBlock<BedrockagentcoreCodeInterpreter_Certificate>>,
    network_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreCodeInterpreter_NetworkConfiguration>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreCodeInterpreter_Certificate_Location_SecretsManager {
    secret_arn: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreCodeInterpreter_Certificate_Location_SecretsManager { }

impl terrars::ToListMappable for BedrockagentcoreCodeInterpreter_Certificate_Location_SecretsManager {
    type O = terrars::BlockAssignable<BedrockagentcoreCodeInterpreter_Certificate_Location_SecretsManager>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreCodeInterpreter_Certificate_Location_SecretsManager {
    #[doc = ""]
    pub secret_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreCodeInterpreter_Certificate_Location_SecretsManager {
    pub fn build(self) -> BedrockagentcoreCodeInterpreter_Certificate_Location_SecretsManager {
        BedrockagentcoreCodeInterpreter_Certificate_Location_SecretsManager { secret_arn: self.secret_arn }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreCodeInterpreter_Certificate_Location_SecretsManagerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreCodeInterpreter_Certificate_Location_SecretsManagerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreCodeInterpreter_Certificate_Location_SecretsManagerRef {
        BedrockagentcoreCodeInterpreter_Certificate_Location_SecretsManagerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreCodeInterpreter_Certificate_Location_SecretsManagerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreCodeInterpreter_Certificate_LocationDynamic {
    secrets_manager: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreCodeInterpreter_Certificate_Location_SecretsManager>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreCodeInterpreter_Certificate_Location {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreCodeInterpreter_Certificate_Location_SecretsManager>>,
    dynamic: BedrockagentcoreCodeInterpreter_Certificate_LocationDynamic,
}

impl BedrockagentcoreCodeInterpreter_Certificate_Location {
    #[doc = "Set the field `secrets_manager`.\n"]
    pub fn set_secrets_manager(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreCodeInterpreter_Certificate_Location_SecretsManager>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secrets_manager = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secrets_manager = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreCodeInterpreter_Certificate_Location {
    type O = terrars::BlockAssignable<BedrockagentcoreCodeInterpreter_Certificate_Location>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreCodeInterpreter_Certificate_Location {}

impl BuildBedrockagentcoreCodeInterpreter_Certificate_Location {
    pub fn build(self) -> BedrockagentcoreCodeInterpreter_Certificate_Location {
        BedrockagentcoreCodeInterpreter_Certificate_Location {
            secrets_manager: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreCodeInterpreter_Certificate_LocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreCodeInterpreter_Certificate_LocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreCodeInterpreter_Certificate_LocationRef {
        BedrockagentcoreCodeInterpreter_Certificate_LocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreCodeInterpreter_Certificate_LocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secrets_manager` after provisioning.\n"]
    pub fn secrets_manager(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreCodeInterpreter_Certificate_Location_SecretsManagerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secrets_manager", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreCodeInterpreter_CertificateDynamic {
    location: std::option::Option<terrars::DynamicBlock<BedrockagentcoreCodeInterpreter_Certificate_Location>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreCodeInterpreter_Certificate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<std::vec::Vec<BedrockagentcoreCodeInterpreter_Certificate_Location>>,
    dynamic: BedrockagentcoreCodeInterpreter_CertificateDynamic,
}

impl BedrockagentcoreCodeInterpreter_Certificate {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreCodeInterpreter_Certificate_Location>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.location = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreCodeInterpreter_Certificate {
    type O = terrars::BlockAssignable<BedrockagentcoreCodeInterpreter_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreCodeInterpreter_Certificate {}

impl BuildBedrockagentcoreCodeInterpreter_Certificate {
    pub fn build(self) -> BedrockagentcoreCodeInterpreter_Certificate {
        BedrockagentcoreCodeInterpreter_Certificate {
            location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreCodeInterpreter_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreCodeInterpreter_CertificateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreCodeInterpreter_CertificateRef {
        BedrockagentcoreCodeInterpreter_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreCodeInterpreter_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::ListRef<BedrockagentcoreCodeInterpreter_Certificate_LocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.location", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreCodeInterpreter_NetworkConfiguration_VpcConfig {
    security_groups: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreCodeInterpreter_NetworkConfiguration_VpcConfig { }

impl terrars::ToListMappable for BedrockagentcoreCodeInterpreter_NetworkConfiguration_VpcConfig {
    type O = terrars::BlockAssignable<BedrockagentcoreCodeInterpreter_NetworkConfiguration_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreCodeInterpreter_NetworkConfiguration_VpcConfig {
    #[doc = ""]
    pub security_groups: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildBedrockagentcoreCodeInterpreter_NetworkConfiguration_VpcConfig {
    pub fn build(self) -> BedrockagentcoreCodeInterpreter_NetworkConfiguration_VpcConfig {
        BedrockagentcoreCodeInterpreter_NetworkConfiguration_VpcConfig {
            security_groups: self.security_groups,
            subnets: self.subnets,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreCodeInterpreter_NetworkConfiguration_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreCodeInterpreter_NetworkConfiguration_VpcConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreCodeInterpreter_NetworkConfiguration_VpcConfigRef {
        BedrockagentcoreCodeInterpreter_NetworkConfiguration_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreCodeInterpreter_NetworkConfiguration_VpcConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreCodeInterpreter_NetworkConfigurationDynamic {
    vpc_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreCodeInterpreter_NetworkConfiguration_VpcConfig>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreCodeInterpreter_NetworkConfiguration {
    network_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<BedrockagentcoreCodeInterpreter_NetworkConfiguration_VpcConfig>>,
    dynamic: BedrockagentcoreCodeInterpreter_NetworkConfigurationDynamic,
}

impl BedrockagentcoreCodeInterpreter_NetworkConfiguration {
    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreCodeInterpreter_NetworkConfiguration_VpcConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreCodeInterpreter_NetworkConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreCodeInterpreter_NetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreCodeInterpreter_NetworkConfiguration {
    #[doc = ""]
    pub network_mode: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreCodeInterpreter_NetworkConfiguration {
    pub fn build(self) -> BedrockagentcoreCodeInterpreter_NetworkConfiguration {
        BedrockagentcoreCodeInterpreter_NetworkConfiguration {
            network_mode: self.network_mode,
            vpc_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreCodeInterpreter_NetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreCodeInterpreter_NetworkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreCodeInterpreter_NetworkConfigurationRef {
        BedrockagentcoreCodeInterpreter_NetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreCodeInterpreter_NetworkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_mode` after provisioning.\n"]
    pub fn network_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<BedrockagentcoreCodeInterpreter_NetworkConfiguration_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreCodeInterpreter_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreCodeInterpreter_Timeouts {
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
}

impl terrars::ToListMappable for BedrockagentcoreCodeInterpreter_Timeouts {
    type O = terrars::BlockAssignable<BedrockagentcoreCodeInterpreter_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreCodeInterpreter_Timeouts {}

impl BuildBedrockagentcoreCodeInterpreter_Timeouts {
    pub fn build(self) -> BedrockagentcoreCodeInterpreter_Timeouts {
        BedrockagentcoreCodeInterpreter_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreCodeInterpreter_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreCodeInterpreter_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreCodeInterpreter_TimeoutsRef {
        BedrockagentcoreCodeInterpreter_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreCodeInterpreter_TimeoutsRef {
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
}
