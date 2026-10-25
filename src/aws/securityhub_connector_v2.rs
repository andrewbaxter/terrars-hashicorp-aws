#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SecurityhubConnectorV2Data_ {
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
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_provider: std::option::Option<std::vec::Vec<SecurityhubConnectorV2_ConnectorProvider>>,
    dynamic: SecurityhubConnectorV2Dynamic,
}

struct SecurityhubConnectorV2_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SecurityhubConnectorV2Data_>,
}

#[derive(Clone)]
pub struct SecurityhubConnectorV2(std::rc::Rc<SecurityhubConnectorV2_>);

impl SecurityhubConnectorV2 {
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

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `connector_provider`.\n"]
    pub fn set_connector_provider(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubConnectorV2_ConnectorProvider>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().connector_provider = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.connector_provider = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connector_id` after provisioning.\n"]
    pub fn connector_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health` after provisioning.\n"]
    pub fn health(&self) -> terrars::ListRef<SecurityhubConnectorV2_HealthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `connector_provider` after provisioning.\n"]
    pub fn connector_provider(&self) -> terrars::ListRef<SecurityhubConnectorV2_ConnectorProviderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_provider", self.extract_ref()))
    }
}

impl terrars::Referable for SecurityhubConnectorV2 {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SecurityhubConnectorV2 { }

impl terrars::ToListMappable for SecurityhubConnectorV2 {
    type O = terrars::ListRef<SecurityhubConnectorV2Ref>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SecurityhubConnectorV2_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_securityhub_connector_v2".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecurityhubConnectorV2 {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubConnectorV2 {
    pub fn build(self, stack: &mut terrars::Stack) -> SecurityhubConnectorV2 {
        let out = SecurityhubConnectorV2(std::rc::Rc::new(SecurityhubConnectorV2_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SecurityhubConnectorV2Data_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                connector_provider: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecurityhubConnectorV2Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubConnectorV2Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SecurityhubConnectorV2Ref {
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

    #[doc = "Get a reference to the value of field `connector_id` after provisioning.\n"]
    pub fn connector_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health` after provisioning.\n"]
    pub fn health(&self) -> terrars::ListRef<SecurityhubConnectorV2_HealthRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `connector_provider` after provisioning.\n"]
    pub fn connector_provider(&self) -> terrars::ListRef<SecurityhubConnectorV2_ConnectorProviderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connector_provider", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubConnectorV2Dynamic {
    connector_provider: std::option::Option<terrars::DynamicBlock<SecurityhubConnectorV2_ConnectorProvider>>,
}

#[derive(Serialize)]
pub struct SecurityhubConnectorV2_Health {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connector_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_checked_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityhubConnectorV2_Health {
    #[doc = "Set the field `connector_status`.\n"]
    pub fn set_connector_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.connector_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_checked_at`.\n"]
    pub fn set_last_checked_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.last_checked_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubConnectorV2_Health {
    type O = terrars::BlockAssignable<SecurityhubConnectorV2_Health>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubConnectorV2_Health {}

impl BuildSecurityhubConnectorV2_Health {
    pub fn build(self) -> SecurityhubConnectorV2_Health {
        SecurityhubConnectorV2_Health {
            connector_status: core::default::Default::default(),
            last_checked_at: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubConnectorV2_HealthRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubConnectorV2_HealthRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubConnectorV2_HealthRef {
        SecurityhubConnectorV2_HealthRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubConnectorV2_HealthRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connector_status` after provisioning.\n"]
    pub fn connector_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_status", self.base))
    }

    #[doc = "Get a reference to the value of field `last_checked_at` after provisioning.\n"]
    pub fn last_checked_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_checked_at", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubConnectorV2_ConnectorProvider_JiraCloud {
    project_key: terrars::PrimField<std::string::String>,
}

impl SecurityhubConnectorV2_ConnectorProvider_JiraCloud { }

impl terrars::ToListMappable for SecurityhubConnectorV2_ConnectorProvider_JiraCloud {
    type O = terrars::BlockAssignable<SecurityhubConnectorV2_ConnectorProvider_JiraCloud>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubConnectorV2_ConnectorProvider_JiraCloud {
    #[doc = ""]
    pub project_key: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubConnectorV2_ConnectorProvider_JiraCloud {
    pub fn build(self) -> SecurityhubConnectorV2_ConnectorProvider_JiraCloud {
        SecurityhubConnectorV2_ConnectorProvider_JiraCloud { project_key: self.project_key }
    }
}

#[allow(dead_code)]
pub struct SecurityhubConnectorV2_ConnectorProvider_JiraCloudRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubConnectorV2_ConnectorProvider_JiraCloudRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubConnectorV2_ConnectorProvider_JiraCloudRef {
        SecurityhubConnectorV2_ConnectorProvider_JiraCloudRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubConnectorV2_ConnectorProvider_JiraCloudRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_status` after provisioning.\n"]
    pub fn auth_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_status", self.base))
    }

    #[doc = "Get a reference to the value of field `auth_url` after provisioning.\n"]
    pub fn auth_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_url", self.base))
    }

    #[doc = "Get a reference to the value of field `cloud_id` after provisioning.\n"]
    pub fn cloud_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_id", self.base))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }

    #[doc = "Get a reference to the value of field `project_key` after provisioning.\n"]
    pub fn project_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_key", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubConnectorV2_ConnectorProvider_ServiceNow {
    instance_name: terrars::PrimField<std::string::String>,
    secret_arn: terrars::PrimField<std::string::String>,
}

impl SecurityhubConnectorV2_ConnectorProvider_ServiceNow { }

impl terrars::ToListMappable for SecurityhubConnectorV2_ConnectorProvider_ServiceNow {
    type O = terrars::BlockAssignable<SecurityhubConnectorV2_ConnectorProvider_ServiceNow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubConnectorV2_ConnectorProvider_ServiceNow {
    #[doc = ""]
    pub instance_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub secret_arn: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubConnectorV2_ConnectorProvider_ServiceNow {
    pub fn build(self) -> SecurityhubConnectorV2_ConnectorProvider_ServiceNow {
        SecurityhubConnectorV2_ConnectorProvider_ServiceNow {
            instance_name: self.instance_name,
            secret_arn: self.secret_arn,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubConnectorV2_ConnectorProvider_ServiceNowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubConnectorV2_ConnectorProvider_ServiceNowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubConnectorV2_ConnectorProvider_ServiceNowRef {
        SecurityhubConnectorV2_ConnectorProvider_ServiceNowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubConnectorV2_ConnectorProvider_ServiceNowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auth_status` after provisioning.\n"]
    pub fn auth_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auth_status", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_name` after provisioning.\n"]
    pub fn instance_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_name", self.base))
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubConnectorV2_ConnectorProviderDynamic {
    jira_cloud: std::option::Option<terrars::DynamicBlock<SecurityhubConnectorV2_ConnectorProvider_JiraCloud>>,
    service_now: std::option::Option<terrars::DynamicBlock<SecurityhubConnectorV2_ConnectorProvider_ServiceNow>>,
}

#[derive(Serialize)]
pub struct SecurityhubConnectorV2_ConnectorProvider {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jira_cloud: std::option::Option<std::vec::Vec<SecurityhubConnectorV2_ConnectorProvider_JiraCloud>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_now: std::option::Option<std::vec::Vec<SecurityhubConnectorV2_ConnectorProvider_ServiceNow>>,
    dynamic: SecurityhubConnectorV2_ConnectorProviderDynamic,
}

impl SecurityhubConnectorV2_ConnectorProvider {
    #[doc = "Set the field `jira_cloud`.\n"]
    pub fn set_jira_cloud(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubConnectorV2_ConnectorProvider_JiraCloud>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.jira_cloud = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.jira_cloud = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_now`.\n"]
    pub fn set_service_now(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubConnectorV2_ConnectorProvider_ServiceNow>>,
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
}

impl terrars::ToListMappable for SecurityhubConnectorV2_ConnectorProvider {
    type O = terrars::BlockAssignable<SecurityhubConnectorV2_ConnectorProvider>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubConnectorV2_ConnectorProvider {}

impl BuildSecurityhubConnectorV2_ConnectorProvider {
    pub fn build(self) -> SecurityhubConnectorV2_ConnectorProvider {
        SecurityhubConnectorV2_ConnectorProvider {
            jira_cloud: core::default::Default::default(),
            service_now: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubConnectorV2_ConnectorProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubConnectorV2_ConnectorProviderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubConnectorV2_ConnectorProviderRef {
        SecurityhubConnectorV2_ConnectorProviderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubConnectorV2_ConnectorProviderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `jira_cloud` after provisioning.\n"]
    pub fn jira_cloud(&self) -> terrars::ListRef<SecurityhubConnectorV2_ConnectorProvider_JiraCloudRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.jira_cloud", self.base))
    }

    #[doc = "Get a reference to the value of field `service_now` after provisioning.\n"]
    pub fn service_now(&self) -> terrars::ListRef<SecurityhubConnectorV2_ConnectorProvider_ServiceNowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_now", self.base))
    }
}
