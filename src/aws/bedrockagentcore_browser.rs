#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentcoreBrowserData_ {
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
    browser_signing: std::option::Option<std::vec::Vec<BedrockagentcoreBrowser_BrowserSigning>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate: std::option::Option<std::vec::Vec<BedrockagentcoreBrowser_Certificate>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enterprise_policy: std::option::Option<std::vec::Vec<BedrockagentcoreBrowser_EnterprisePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std::option::Option<std::vec::Vec<BedrockagentcoreBrowser_NetworkConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recording: std::option::Option<std::vec::Vec<BedrockagentcoreBrowser_Recording>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockagentcoreBrowser_Timeouts>,
    dynamic: BedrockagentcoreBrowserDynamic,
}

struct BedrockagentcoreBrowser_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentcoreBrowserData_>,
}

#[derive(Clone)]
pub struct BedrockagentcoreBrowser(std::rc::Rc<BedrockagentcoreBrowser_>);

impl BedrockagentcoreBrowser {
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

    #[doc = "Set the field `browser_signing`.\n"]
    pub fn set_browser_signing(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreBrowser_BrowserSigning>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().browser_signing = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.browser_signing = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `certificate`.\n"]
    pub fn set_certificate(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreBrowser_Certificate>>,
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

    #[doc = "Set the field `enterprise_policy`.\n"]
    pub fn set_enterprise_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreBrowser_EnterprisePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().enterprise_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.enterprise_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreBrowser_NetworkConfiguration>>,
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

    #[doc = "Set the field `recording`.\n"]
    pub fn set_recording(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreBrowser_Recording>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().recording = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.recording = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockagentcoreBrowser_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `browser_arn` after provisioning.\n"]
    pub fn browser_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.browser_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `browser_id` after provisioning.\n"]
    pub fn browser_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.browser_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `browser_signing` after provisioning.\n"]
    pub fn browser_signing(&self) -> terrars::ListRef<BedrockagentcoreBrowser_BrowserSigningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.browser_signing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::ListRef<BedrockagentcoreBrowser_CertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enterprise_policy` after provisioning.\n"]
    pub fn enterprise_policy(&self) -> terrars::ListRef<BedrockagentcoreBrowser_EnterprisePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enterprise_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<BedrockagentcoreBrowser_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recording` after provisioning.\n"]
    pub fn recording(&self) -> terrars::ListRef<BedrockagentcoreBrowser_RecordingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recording", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreBrowser_TimeoutsRef {
        BedrockagentcoreBrowser_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BedrockagentcoreBrowser {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentcoreBrowser { }

impl terrars::ToListMappable for BedrockagentcoreBrowser {
    type O = terrars::ListRef<BedrockagentcoreBrowserRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentcoreBrowser_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagentcore_browser".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentcoreBrowser {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreBrowser {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentcoreBrowser {
        let out = BedrockagentcoreBrowser(std::rc::Rc::new(BedrockagentcoreBrowser_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockagentcoreBrowserData_ {
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
                browser_signing: core::default::Default::default(),
                certificate: core::default::Default::default(),
                enterprise_policy: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                recording: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentcoreBrowserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreBrowserRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentcoreBrowserRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `browser_arn` after provisioning.\n"]
    pub fn browser_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.browser_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `browser_id` after provisioning.\n"]
    pub fn browser_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.browser_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `browser_signing` after provisioning.\n"]
    pub fn browser_signing(&self) -> terrars::ListRef<BedrockagentcoreBrowser_BrowserSigningRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.browser_signing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::ListRef<BedrockagentcoreBrowser_CertificateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enterprise_policy` after provisioning.\n"]
    pub fn enterprise_policy(&self) -> terrars::ListRef<BedrockagentcoreBrowser_EnterprisePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enterprise_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<BedrockagentcoreBrowser_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recording` after provisioning.\n"]
    pub fn recording(&self) -> terrars::ListRef<BedrockagentcoreBrowser_RecordingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.recording", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreBrowser_TimeoutsRef {
        BedrockagentcoreBrowser_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreBrowserDynamic {
    browser_signing: std::option::Option<terrars::DynamicBlock<BedrockagentcoreBrowser_BrowserSigning>>,
    certificate: std::option::Option<terrars::DynamicBlock<BedrockagentcoreBrowser_Certificate>>,
    enterprise_policy: std::option::Option<terrars::DynamicBlock<BedrockagentcoreBrowser_EnterprisePolicy>>,
    network_configuration: std::option::Option<terrars::DynamicBlock<BedrockagentcoreBrowser_NetworkConfiguration>>,
    recording: std::option::Option<terrars::DynamicBlock<BedrockagentcoreBrowser_Recording>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreBrowser_BrowserSigning {
    enabled: terrars::PrimField<bool>,
}

impl BedrockagentcoreBrowser_BrowserSigning { }

impl terrars::ToListMappable for BedrockagentcoreBrowser_BrowserSigning {
    type O = terrars::BlockAssignable<BedrockagentcoreBrowser_BrowserSigning>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreBrowser_BrowserSigning {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildBedrockagentcoreBrowser_BrowserSigning {
    pub fn build(self) -> BedrockagentcoreBrowser_BrowserSigning {
        BedrockagentcoreBrowser_BrowserSigning { enabled: self.enabled }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreBrowser_BrowserSigningRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreBrowser_BrowserSigningRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreBrowser_BrowserSigningRef {
        BedrockagentcoreBrowser_BrowserSigningRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreBrowser_BrowserSigningRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreBrowser_Certificate_Location_SecretsManager {
    secret_arn: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreBrowser_Certificate_Location_SecretsManager { }

impl terrars::ToListMappable for BedrockagentcoreBrowser_Certificate_Location_SecretsManager {
    type O = terrars::BlockAssignable<BedrockagentcoreBrowser_Certificate_Location_SecretsManager>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreBrowser_Certificate_Location_SecretsManager {
    #[doc = ""]
    pub secret_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreBrowser_Certificate_Location_SecretsManager {
    pub fn build(self) -> BedrockagentcoreBrowser_Certificate_Location_SecretsManager {
        BedrockagentcoreBrowser_Certificate_Location_SecretsManager { secret_arn: self.secret_arn }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreBrowser_Certificate_Location_SecretsManagerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreBrowser_Certificate_Location_SecretsManagerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreBrowser_Certificate_Location_SecretsManagerRef {
        BedrockagentcoreBrowser_Certificate_Location_SecretsManagerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreBrowser_Certificate_Location_SecretsManagerRef {
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
struct BedrockagentcoreBrowser_Certificate_LocationDynamic {
    secrets_manager: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreBrowser_Certificate_Location_SecretsManager>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreBrowser_Certificate_Location {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secrets_manager: std::option::Option<std::vec::Vec<BedrockagentcoreBrowser_Certificate_Location_SecretsManager>>,
    dynamic: BedrockagentcoreBrowser_Certificate_LocationDynamic,
}

impl BedrockagentcoreBrowser_Certificate_Location {
    #[doc = "Set the field `secrets_manager`.\n"]
    pub fn set_secrets_manager(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreBrowser_Certificate_Location_SecretsManager>>,
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

impl terrars::ToListMappable for BedrockagentcoreBrowser_Certificate_Location {
    type O = terrars::BlockAssignable<BedrockagentcoreBrowser_Certificate_Location>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreBrowser_Certificate_Location {}

impl BuildBedrockagentcoreBrowser_Certificate_Location {
    pub fn build(self) -> BedrockagentcoreBrowser_Certificate_Location {
        BedrockagentcoreBrowser_Certificate_Location {
            secrets_manager: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreBrowser_Certificate_LocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreBrowser_Certificate_LocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreBrowser_Certificate_LocationRef {
        BedrockagentcoreBrowser_Certificate_LocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreBrowser_Certificate_LocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secrets_manager` after provisioning.\n"]
    pub fn secrets_manager(&self) -> terrars::ListRef<BedrockagentcoreBrowser_Certificate_Location_SecretsManagerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secrets_manager", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreBrowser_CertificateDynamic {
    location: std::option::Option<terrars::DynamicBlock<BedrockagentcoreBrowser_Certificate_Location>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreBrowser_Certificate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<std::vec::Vec<BedrockagentcoreBrowser_Certificate_Location>>,
    dynamic: BedrockagentcoreBrowser_CertificateDynamic,
}

impl BedrockagentcoreBrowser_Certificate {
    #[doc = "Set the field `location`.\n"]
    pub fn set_location(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreBrowser_Certificate_Location>>,
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

impl terrars::ToListMappable for BedrockagentcoreBrowser_Certificate {
    type O = terrars::BlockAssignable<BedrockagentcoreBrowser_Certificate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreBrowser_Certificate {}

impl BuildBedrockagentcoreBrowser_Certificate {
    pub fn build(self) -> BedrockagentcoreBrowser_Certificate {
        BedrockagentcoreBrowser_Certificate {
            location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreBrowser_CertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreBrowser_CertificateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreBrowser_CertificateRef {
        BedrockagentcoreBrowser_CertificateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreBrowser_CertificateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::ListRef<BedrockagentcoreBrowser_Certificate_LocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.location", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreBrowser_EnterprisePolicy_Location_S3 {
    bucket: terrars::PrimField<std::string::String>,
    prefix: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreBrowser_EnterprisePolicy_Location_S3 {
    #[doc = "Set the field `version_id`.\n"]
    pub fn set_version_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreBrowser_EnterprisePolicy_Location_S3 {
    type O = terrars::BlockAssignable<BedrockagentcoreBrowser_EnterprisePolicy_Location_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreBrowser_EnterprisePolicy_Location_S3 {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub prefix: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreBrowser_EnterprisePolicy_Location_S3 {
    pub fn build(self) -> BedrockagentcoreBrowser_EnterprisePolicy_Location_S3 {
        BedrockagentcoreBrowser_EnterprisePolicy_Location_S3 {
            bucket: self.bucket,
            prefix: self.prefix,
            version_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreBrowser_EnterprisePolicy_Location_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreBrowser_EnterprisePolicy_Location_S3Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreBrowser_EnterprisePolicy_Location_S3Ref {
        BedrockagentcoreBrowser_EnterprisePolicy_Location_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreBrowser_EnterprisePolicy_Location_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `version_id` after provisioning.\n"]
    pub fn version_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreBrowser_EnterprisePolicy_LocationDynamic {
    s3: std::option::Option<terrars::DynamicBlock<BedrockagentcoreBrowser_EnterprisePolicy_Location_S3>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreBrowser_EnterprisePolicy_Location {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<BedrockagentcoreBrowser_EnterprisePolicy_Location_S3>>,
    dynamic: BedrockagentcoreBrowser_EnterprisePolicy_LocationDynamic,
}

impl BedrockagentcoreBrowser_EnterprisePolicy_Location {
    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreBrowser_EnterprisePolicy_Location_S3>>,
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
}

impl terrars::ToListMappable for BedrockagentcoreBrowser_EnterprisePolicy_Location {
    type O = terrars::BlockAssignable<BedrockagentcoreBrowser_EnterprisePolicy_Location>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreBrowser_EnterprisePolicy_Location {}

impl BuildBedrockagentcoreBrowser_EnterprisePolicy_Location {
    pub fn build(self) -> BedrockagentcoreBrowser_EnterprisePolicy_Location {
        BedrockagentcoreBrowser_EnterprisePolicy_Location {
            s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreBrowser_EnterprisePolicy_LocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreBrowser_EnterprisePolicy_LocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreBrowser_EnterprisePolicy_LocationRef {
        BedrockagentcoreBrowser_EnterprisePolicy_LocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreBrowser_EnterprisePolicy_LocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<BedrockagentcoreBrowser_EnterprisePolicy_Location_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreBrowser_EnterprisePolicyDynamic {
    location: std::option::Option<terrars::DynamicBlock<BedrockagentcoreBrowser_EnterprisePolicy_Location>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreBrowser_EnterprisePolicy {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<std::vec::Vec<BedrockagentcoreBrowser_EnterprisePolicy_Location>>,
    dynamic: BedrockagentcoreBrowser_EnterprisePolicyDynamic,
}

impl BedrockagentcoreBrowser_EnterprisePolicy {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreBrowser_EnterprisePolicy_Location>>,
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

impl terrars::ToListMappable for BedrockagentcoreBrowser_EnterprisePolicy {
    type O = terrars::BlockAssignable<BedrockagentcoreBrowser_EnterprisePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreBrowser_EnterprisePolicy {}

impl BuildBedrockagentcoreBrowser_EnterprisePolicy {
    pub fn build(self) -> BedrockagentcoreBrowser_EnterprisePolicy {
        BedrockagentcoreBrowser_EnterprisePolicy {
            type_: core::default::Default::default(),
            location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreBrowser_EnterprisePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreBrowser_EnterprisePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreBrowser_EnterprisePolicyRef {
        BedrockagentcoreBrowser_EnterprisePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreBrowser_EnterprisePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::ListRef<BedrockagentcoreBrowser_EnterprisePolicy_LocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.location", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreBrowser_NetworkConfiguration_VpcConfig {
    security_groups: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreBrowser_NetworkConfiguration_VpcConfig { }

impl terrars::ToListMappable for BedrockagentcoreBrowser_NetworkConfiguration_VpcConfig {
    type O = terrars::BlockAssignable<BedrockagentcoreBrowser_NetworkConfiguration_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreBrowser_NetworkConfiguration_VpcConfig {
    #[doc = ""]
    pub security_groups: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildBedrockagentcoreBrowser_NetworkConfiguration_VpcConfig {
    pub fn build(self) -> BedrockagentcoreBrowser_NetworkConfiguration_VpcConfig {
        BedrockagentcoreBrowser_NetworkConfiguration_VpcConfig {
            security_groups: self.security_groups,
            subnets: self.subnets,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreBrowser_NetworkConfiguration_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreBrowser_NetworkConfiguration_VpcConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreBrowser_NetworkConfiguration_VpcConfigRef {
        BedrockagentcoreBrowser_NetworkConfiguration_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreBrowser_NetworkConfiguration_VpcConfigRef {
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
struct BedrockagentcoreBrowser_NetworkConfigurationDynamic {
    vpc_config: std::option::Option<terrars::DynamicBlock<BedrockagentcoreBrowser_NetworkConfiguration_VpcConfig>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreBrowser_NetworkConfiguration {
    network_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<BedrockagentcoreBrowser_NetworkConfiguration_VpcConfig>>,
    dynamic: BedrockagentcoreBrowser_NetworkConfigurationDynamic,
}

impl BedrockagentcoreBrowser_NetworkConfiguration {
    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreBrowser_NetworkConfiguration_VpcConfig>>,
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

impl terrars::ToListMappable for BedrockagentcoreBrowser_NetworkConfiguration {
    type O = terrars::BlockAssignable<BedrockagentcoreBrowser_NetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreBrowser_NetworkConfiguration {
    #[doc = ""]
    pub network_mode: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreBrowser_NetworkConfiguration {
    pub fn build(self) -> BedrockagentcoreBrowser_NetworkConfiguration {
        BedrockagentcoreBrowser_NetworkConfiguration {
            network_mode: self.network_mode,
            vpc_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreBrowser_NetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreBrowser_NetworkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreBrowser_NetworkConfigurationRef {
        BedrockagentcoreBrowser_NetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreBrowser_NetworkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_mode` after provisioning.\n"]
    pub fn network_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<BedrockagentcoreBrowser_NetworkConfiguration_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreBrowser_Recording_S3Location {
    bucket: terrars::PrimField<std::string::String>,
    prefix: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreBrowser_Recording_S3Location { }

impl terrars::ToListMappable for BedrockagentcoreBrowser_Recording_S3Location {
    type O = terrars::BlockAssignable<BedrockagentcoreBrowser_Recording_S3Location>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreBrowser_Recording_S3Location {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub prefix: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreBrowser_Recording_S3Location {
    pub fn build(self) -> BedrockagentcoreBrowser_Recording_S3Location {
        BedrockagentcoreBrowser_Recording_S3Location {
            bucket: self.bucket,
            prefix: self.prefix,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreBrowser_Recording_S3LocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreBrowser_Recording_S3LocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreBrowser_Recording_S3LocationRef {
        BedrockagentcoreBrowser_Recording_S3LocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreBrowser_Recording_S3LocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreBrowser_RecordingDynamic {
    s3_location: std::option::Option<terrars::DynamicBlock<BedrockagentcoreBrowser_Recording_S3Location>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreBrowser_Recording {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_location: std::option::Option<std::vec::Vec<BedrockagentcoreBrowser_Recording_S3Location>>,
    dynamic: BedrockagentcoreBrowser_RecordingDynamic,
}

impl BedrockagentcoreBrowser_Recording {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_location`.\n"]
    pub fn set_s3_location(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreBrowser_Recording_S3Location>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_location = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_location = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreBrowser_Recording {
    type O = terrars::BlockAssignable<BedrockagentcoreBrowser_Recording>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreBrowser_Recording {}

impl BuildBedrockagentcoreBrowser_Recording {
    pub fn build(self) -> BedrockagentcoreBrowser_Recording {
        BedrockagentcoreBrowser_Recording {
            enabled: core::default::Default::default(),
            s3_location: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreBrowser_RecordingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreBrowser_RecordingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreBrowser_RecordingRef {
        BedrockagentcoreBrowser_RecordingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreBrowser_RecordingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_location` after provisioning.\n"]
    pub fn s3_location(&self) -> terrars::ListRef<BedrockagentcoreBrowser_Recording_S3LocationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_location", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreBrowser_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreBrowser_Timeouts {
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

impl terrars::ToListMappable for BedrockagentcoreBrowser_Timeouts {
    type O = terrars::BlockAssignable<BedrockagentcoreBrowser_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreBrowser_Timeouts {}

impl BuildBedrockagentcoreBrowser_Timeouts {
    pub fn build(self) -> BedrockagentcoreBrowser_Timeouts {
        BedrockagentcoreBrowser_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreBrowser_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreBrowser_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreBrowser_TimeoutsRef {
        BedrockagentcoreBrowser_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreBrowser_TimeoutsRef {
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
