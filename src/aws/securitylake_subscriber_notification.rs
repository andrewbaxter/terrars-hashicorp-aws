#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SecuritylakeSubscriberNotificationData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    subscriber_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration: std::option::Option<std::vec::Vec<SecuritylakeSubscriberNotification_Configuration>>,
    dynamic: SecuritylakeSubscriberNotificationDynamic,
}

struct SecuritylakeSubscriberNotification_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SecuritylakeSubscriberNotificationData_>,
}

#[derive(Clone)]
pub struct SecuritylakeSubscriberNotification(std::rc::Rc<SecuritylakeSubscriberNotification_>);

impl SecuritylakeSubscriberNotification {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configuration`.\n"]
    pub fn set_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecuritylakeSubscriberNotification_Configuration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subscriber_endpoint` after provisioning.\n"]
    pub fn subscriber_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriber_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subscriber_id` after provisioning.\n"]
    pub fn subscriber_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriber_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<SecuritylakeSubscriberNotification_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

impl terrars::Referable for SecuritylakeSubscriberNotification {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SecuritylakeSubscriberNotification { }

impl terrars::ToListMappable for SecuritylakeSubscriberNotification {
    type O = terrars::ListRef<SecuritylakeSubscriberNotificationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SecuritylakeSubscriberNotification_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_securitylake_subscriber_notification".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecuritylakeSubscriberNotification {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub subscriber_id: terrars::PrimField<std::string::String>,
}

impl BuildSecuritylakeSubscriberNotification {
    pub fn build(self, stack: &mut terrars::Stack) -> SecuritylakeSubscriberNotification {
        let out = SecuritylakeSubscriberNotification(std::rc::Rc::new(SecuritylakeSubscriberNotification_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SecuritylakeSubscriberNotificationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                region: core::default::Default::default(),
                subscriber_id: self.subscriber_id,
                configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecuritylakeSubscriberNotificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeSubscriberNotificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SecuritylakeSubscriberNotificationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subscriber_endpoint` after provisioning.\n"]
    pub fn subscriber_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriber_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subscriber_id` after provisioning.\n"]
    pub fn subscriber_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscriber_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::ListRef<SecuritylakeSubscriberNotification_ConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SecuritylakeSubscriberNotificationDynamic {
    configuration: std::option::Option<terrars::DynamicBlock<SecuritylakeSubscriberNotification_Configuration>>,
}

#[derive(Serialize)]
pub struct SecuritylakeSubscriberNotification_Configuration_HttpsNotificationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_api_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_api_key_value: std::option::Option<terrars::PrimField<std::string::String>>,
    endpoint: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_method: std::option::Option<terrars::PrimField<std::string::String>>,
    target_role_arn: terrars::PrimField<std::string::String>,
}

impl SecuritylakeSubscriberNotification_Configuration_HttpsNotificationConfiguration {
    #[doc = "Set the field `authorization_api_key_name`.\n"]
    pub fn set_authorization_api_key_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authorization_api_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorization_api_key_value`.\n"]
    pub fn set_authorization_api_key_value(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.authorization_api_key_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_method`.\n"]
    pub fn set_http_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_method = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecuritylakeSubscriberNotification_Configuration_HttpsNotificationConfiguration {
    type O =
        terrars::BlockAssignable<SecuritylakeSubscriberNotification_Configuration_HttpsNotificationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeSubscriberNotification_Configuration_HttpsNotificationConfiguration {
    #[doc = ""]
    pub endpoint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildSecuritylakeSubscriberNotification_Configuration_HttpsNotificationConfiguration {
    pub fn build(self) -> SecuritylakeSubscriberNotification_Configuration_HttpsNotificationConfiguration {
        SecuritylakeSubscriberNotification_Configuration_HttpsNotificationConfiguration {
            authorization_api_key_name: core::default::Default::default(),
            authorization_api_key_value: core::default::Default::default(),
            endpoint: self.endpoint,
            http_method: core::default::Default::default(),
            target_role_arn: self.target_role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeSubscriberNotification_Configuration_HttpsNotificationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeSubscriberNotification_Configuration_HttpsNotificationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecuritylakeSubscriberNotification_Configuration_HttpsNotificationConfigurationRef {
        SecuritylakeSubscriberNotification_Configuration_HttpsNotificationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeSubscriberNotification_Configuration_HttpsNotificationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_api_key_name` after provisioning.\n"]
    pub fn authorization_api_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_api_key_name", self.base))
    }

    #[doc = "Get a reference to the value of field `authorization_api_key_value` after provisioning.\n"]
    pub fn authorization_api_key_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authorization_api_key_value", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `http_method` after provisioning.\n"]
    pub fn http_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_method", self.base))
    }

    #[doc = "Get a reference to the value of field `target_role_arn` after provisioning.\n"]
    pub fn target_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SecuritylakeSubscriberNotification_Configuration_SqsNotificationConfiguration {}

impl SecuritylakeSubscriberNotification_Configuration_SqsNotificationConfiguration { }

impl terrars::ToListMappable for SecuritylakeSubscriberNotification_Configuration_SqsNotificationConfiguration {
    type O = terrars::BlockAssignable<SecuritylakeSubscriberNotification_Configuration_SqsNotificationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeSubscriberNotification_Configuration_SqsNotificationConfiguration {}

impl BuildSecuritylakeSubscriberNotification_Configuration_SqsNotificationConfiguration {
    pub fn build(self) -> SecuritylakeSubscriberNotification_Configuration_SqsNotificationConfiguration {
        SecuritylakeSubscriberNotification_Configuration_SqsNotificationConfiguration {}
    }
}

#[allow(dead_code)]
pub struct SecuritylakeSubscriberNotification_Configuration_SqsNotificationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeSubscriberNotification_Configuration_SqsNotificationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecuritylakeSubscriberNotification_Configuration_SqsNotificationConfigurationRef {
        SecuritylakeSubscriberNotification_Configuration_SqsNotificationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeSubscriberNotification_Configuration_SqsNotificationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct SecuritylakeSubscriberNotification_ConfigurationDynamic {
    https_notification_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<SecuritylakeSubscriberNotification_Configuration_HttpsNotificationConfiguration>>,
    sqs_notification_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<SecuritylakeSubscriberNotification_Configuration_SqsNotificationConfiguration>>,
}

#[derive(Serialize)]
pub struct SecuritylakeSubscriberNotification_Configuration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    https_notification_configuration: std
    ::option
    ::Option<std::vec::Vec<SecuritylakeSubscriberNotification_Configuration_HttpsNotificationConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sqs_notification_configuration: std
    ::option
    ::Option<std::vec::Vec<SecuritylakeSubscriberNotification_Configuration_SqsNotificationConfiguration>>,
    dynamic: SecuritylakeSubscriberNotification_ConfigurationDynamic,
}

impl SecuritylakeSubscriberNotification_Configuration {
    #[doc = "Set the field `https_notification_configuration`.\n"]
    pub fn set_https_notification_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecuritylakeSubscriberNotification_Configuration_HttpsNotificationConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.https_notification_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.https_notification_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sqs_notification_configuration`.\n"]
    pub fn set_sqs_notification_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SecuritylakeSubscriberNotification_Configuration_SqsNotificationConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sqs_notification_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sqs_notification_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecuritylakeSubscriberNotification_Configuration {
    type O = terrars::BlockAssignable<SecuritylakeSubscriberNotification_Configuration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecuritylakeSubscriberNotification_Configuration {}

impl BuildSecuritylakeSubscriberNotification_Configuration {
    pub fn build(self) -> SecuritylakeSubscriberNotification_Configuration {
        SecuritylakeSubscriberNotification_Configuration {
            https_notification_configuration: core::default::Default::default(),
            sqs_notification_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecuritylakeSubscriberNotification_ConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecuritylakeSubscriberNotification_ConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecuritylakeSubscriberNotification_ConfigurationRef {
        SecuritylakeSubscriberNotification_ConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecuritylakeSubscriberNotification_ConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `https_notification_configuration` after provisioning.\n"]
    pub fn https_notification_configuration(
        &self,
    ) -> terrars::ListRef<SecuritylakeSubscriberNotification_Configuration_HttpsNotificationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.https_notification_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `sqs_notification_configuration` after provisioning.\n"]
    pub fn sqs_notification_configuration(
        &self,
    ) -> terrars::ListRef<SecuritylakeSubscriberNotification_Configuration_SqsNotificationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sqs_notification_configuration", self.base))
    }
}
