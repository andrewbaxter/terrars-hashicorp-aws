#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MailmanagerIngressPointData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    rule_set_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_to_update: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    traffic_policy_id: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_point_configuration: std::option::Option<std::vec::Vec<MailmanagerIngressPoint_IngressPointConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std::option::Option<std::vec::Vec<MailmanagerIngressPoint_NetworkConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MailmanagerIngressPoint_Timeouts>,
    dynamic: MailmanagerIngressPointDynamic,
}

struct MailmanagerIngressPoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MailmanagerIngressPointData_>,
}

#[derive(Clone)]
pub struct MailmanagerIngressPoint(std::rc::Rc<MailmanagerIngressPoint_>);

impl MailmanagerIngressPoint {
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

    #[doc = "Set the field `status_to_update`.\n"]
    pub fn set_status_to_update(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status_to_update = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tls_policy`.\n"]
    pub fn set_tls_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tls_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ingress_point_configuration`.\n"]
    pub fn set_ingress_point_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerIngressPoint_IngressPointConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ingress_point_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ingress_point_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerIngressPoint_NetworkConfiguration>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<MailmanagerIngressPoint_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `a_record` after provisioning.\n"]
    pub fn a_record(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.a_record", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_timestamp` after provisioning.\n"]
    pub fn created_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_timestamp` after provisioning.\n"]
    pub fn last_updated_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_timestamp", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `rule_set_id` after provisioning.\n"]
    pub fn rule_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_set_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_to_update` after provisioning.\n"]
    pub fn status_to_update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_to_update", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_policy` after provisioning.\n"]
    pub fn tls_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_policy_id` after provisioning.\n"]
    pub fn traffic_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_policy_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress_point_configuration` after provisioning.\n"]
    pub fn ingress_point_configuration(&self) -> terrars::ListRef<MailmanagerIngressPoint_IngressPointConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_point_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<MailmanagerIngressPoint_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MailmanagerIngressPoint_TimeoutsRef {
        MailmanagerIngressPoint_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for MailmanagerIngressPoint {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MailmanagerIngressPoint { }

impl terrars::ToListMappable for MailmanagerIngressPoint {
    type O = terrars::ListRef<MailmanagerIngressPointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MailmanagerIngressPoint_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_mailmanager_ingress_point".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMailmanagerIngressPoint {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule_set_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub traffic_policy_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerIngressPoint {
    pub fn build(self, stack: &mut terrars::Stack) -> MailmanagerIngressPoint {
        let out = MailmanagerIngressPoint(std::rc::Rc::new(MailmanagerIngressPoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MailmanagerIngressPointData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                rule_set_id: self.rule_set_id,
                status_to_update: core::default::Default::default(),
                tags: core::default::Default::default(),
                tls_policy: core::default::Default::default(),
                traffic_policy_id: self.traffic_policy_id,
                type_: self.type_,
                ingress_point_configuration: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MailmanagerIngressPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerIngressPointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MailmanagerIngressPointRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `a_record` after provisioning.\n"]
    pub fn a_record(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.a_record", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_timestamp` after provisioning.\n"]
    pub fn created_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_timestamp` after provisioning.\n"]
    pub fn last_updated_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_timestamp", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `rule_set_id` after provisioning.\n"]
    pub fn rule_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_set_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_to_update` after provisioning.\n"]
    pub fn status_to_update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_to_update", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_policy` after provisioning.\n"]
    pub fn tls_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_policy_id` after provisioning.\n"]
    pub fn traffic_policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.traffic_policy_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress_point_configuration` after provisioning.\n"]
    pub fn ingress_point_configuration(&self) -> terrars::ListRef<MailmanagerIngressPoint_IngressPointConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_point_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<MailmanagerIngressPoint_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MailmanagerIngressPoint_TimeoutsRef {
        MailmanagerIngressPoint_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerIngressPointDynamic {
    ingress_point_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerIngressPoint_IngressPointConfiguration>>,
    network_configuration: std::option::Option<terrars::DynamicBlock<MailmanagerIngressPoint_NetworkConfiguration>>,
}

#[derive(Serialize)]
pub struct MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration_TrustStore {
    ca_content: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crl_content: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration_TrustStore {
    #[doc = "Set the field `crl_content`.\n"]
    pub fn set_crl_content(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.crl_content = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration_TrustStore {
    type O =
        terrars::BlockAssignable<MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration_TrustStore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration_TrustStore {
    #[doc = ""]
    pub ca_content: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration_TrustStore {
    pub fn build(self) -> MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration_TrustStore {
        MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration_TrustStore {
            ca_content: self.ca_content,
            crl_content: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration_TrustStoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration_TrustStoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration_TrustStoreRef {
        MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration_TrustStoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration_TrustStoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ca_content` after provisioning.\n"]
    pub fn ca_content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_content", self.base))
    }

    #[doc = "Get a reference to the value of field `crl_content` after provisioning.\n"]
    pub fn crl_content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.crl_content", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfigurationDynamic {
    trust_store: std
    ::option
    ::Option<
        terrars::DynamicBlock<MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration_TrustStore>,
    >,
}

#[derive(Serialize)]
pub struct MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_store: std
    ::option
    ::Option<std::vec::Vec<MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration_TrustStore>>,
    dynamic: MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfigurationDynamic,
}

impl MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration {
    #[doc = "Set the field `trust_store`.\n"]
    pub fn set_trust_store(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration_TrustStore,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.trust_store = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.trust_store = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration {
    type O = terrars::BlockAssignable<MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration {}

impl BuildMailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration {
    pub fn build(self) -> MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration {
        MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration {
            trust_store: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfigurationRef {
        MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `trust_store` after provisioning.\n"]
    pub fn trust_store(
        &self,
    ) -> terrars::ListRef<MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration_TrustStoreRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trust_store", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerIngressPoint_IngressPointConfigurationDynamic {
    tls_auth_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration>>,
}

#[derive(Serialize)]
pub struct MailmanagerIngressPoint_IngressPointConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    smtp_password_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    smtp_password_wo_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_auth_configuration: std
    ::option
    ::Option<std::vec::Vec<MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration>>,
    dynamic: MailmanagerIngressPoint_IngressPointConfigurationDynamic,
}

impl MailmanagerIngressPoint_IngressPointConfiguration {
    #[doc = "Set the field `secret_arn`.\n"]
    pub fn set_secret_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.secret_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `smtp_password_wo`.\n"]
    pub fn set_smtp_password_wo(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.smtp_password_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `smtp_password_wo_version`.\n"]
    pub fn set_smtp_password_wo_version(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.smtp_password_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_auth_configuration`.\n"]
    pub fn set_tls_auth_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tls_auth_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tls_auth_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerIngressPoint_IngressPointConfiguration {
    type O = terrars::BlockAssignable<MailmanagerIngressPoint_IngressPointConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerIngressPoint_IngressPointConfiguration {}

impl BuildMailmanagerIngressPoint_IngressPointConfiguration {
    pub fn build(self) -> MailmanagerIngressPoint_IngressPointConfiguration {
        MailmanagerIngressPoint_IngressPointConfiguration {
            secret_arn: core::default::Default::default(),
            smtp_password_wo: core::default::Default::default(),
            smtp_password_wo_version: core::default::Default::default(),
            tls_auth_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerIngressPoint_IngressPointConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerIngressPoint_IngressPointConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerIngressPoint_IngressPointConfigurationRef {
        MailmanagerIngressPoint_IngressPointConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerIngressPoint_IngressPointConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `secret_arn` after provisioning.\n"]
    pub fn secret_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `smtp_password_wo` after provisioning.\n"]
    pub fn smtp_password_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.smtp_password_wo", self.base))
    }

    #[doc = "Get a reference to the value of field `smtp_password_wo_version` after provisioning.\n"]
    pub fn smtp_password_wo_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.smtp_password_wo_version", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_auth_configuration` after provisioning.\n"]
    pub fn tls_auth_configuration(
        &self,
    ) -> terrars::ListRef<MailmanagerIngressPoint_IngressPointConfiguration_TlsAuthConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_auth_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct MailmanagerIngressPoint_NetworkConfiguration_PrivateNetworkConfiguration {
    vpc_endpoint_id: terrars::PrimField<std::string::String>,
}

impl MailmanagerIngressPoint_NetworkConfiguration_PrivateNetworkConfiguration { }

impl terrars::ToListMappable for MailmanagerIngressPoint_NetworkConfiguration_PrivateNetworkConfiguration {
    type O = terrars::BlockAssignable<MailmanagerIngressPoint_NetworkConfiguration_PrivateNetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerIngressPoint_NetworkConfiguration_PrivateNetworkConfiguration {
    #[doc = ""]
    pub vpc_endpoint_id: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerIngressPoint_NetworkConfiguration_PrivateNetworkConfiguration {
    pub fn build(self) -> MailmanagerIngressPoint_NetworkConfiguration_PrivateNetworkConfiguration {
        MailmanagerIngressPoint_NetworkConfiguration_PrivateNetworkConfiguration {
            vpc_endpoint_id: self.vpc_endpoint_id,
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerIngressPoint_NetworkConfiguration_PrivateNetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerIngressPoint_NetworkConfiguration_PrivateNetworkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerIngressPoint_NetworkConfiguration_PrivateNetworkConfigurationRef {
        MailmanagerIngressPoint_NetworkConfiguration_PrivateNetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerIngressPoint_NetworkConfiguration_PrivateNetworkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_id", self.base))
    }
}

#[derive(Serialize)]
pub struct MailmanagerIngressPoint_NetworkConfiguration_PublicNetworkConfiguration {
    ip_type: terrars::PrimField<std::string::String>,
}

impl MailmanagerIngressPoint_NetworkConfiguration_PublicNetworkConfiguration { }

impl terrars::ToListMappable for MailmanagerIngressPoint_NetworkConfiguration_PublicNetworkConfiguration {
    type O = terrars::BlockAssignable<MailmanagerIngressPoint_NetworkConfiguration_PublicNetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerIngressPoint_NetworkConfiguration_PublicNetworkConfiguration {
    #[doc = ""]
    pub ip_type: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerIngressPoint_NetworkConfiguration_PublicNetworkConfiguration {
    pub fn build(self) -> MailmanagerIngressPoint_NetworkConfiguration_PublicNetworkConfiguration {
        MailmanagerIngressPoint_NetworkConfiguration_PublicNetworkConfiguration { ip_type: self.ip_type }
    }
}

#[allow(dead_code)]
pub struct MailmanagerIngressPoint_NetworkConfiguration_PublicNetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerIngressPoint_NetworkConfiguration_PublicNetworkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerIngressPoint_NetworkConfiguration_PublicNetworkConfigurationRef {
        MailmanagerIngressPoint_NetworkConfiguration_PublicNetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerIngressPoint_NetworkConfiguration_PublicNetworkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_type` after provisioning.\n"]
    pub fn ip_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerIngressPoint_NetworkConfigurationDynamic {
    private_network_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerIngressPoint_NetworkConfiguration_PrivateNetworkConfiguration>>,
    public_network_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerIngressPoint_NetworkConfiguration_PublicNetworkConfiguration>>,
}

#[derive(Serialize)]
pub struct MailmanagerIngressPoint_NetworkConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_network_configuration: std
    ::option
    ::Option<std::vec::Vec<MailmanagerIngressPoint_NetworkConfiguration_PrivateNetworkConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_network_configuration: std
    ::option
    ::Option<std::vec::Vec<MailmanagerIngressPoint_NetworkConfiguration_PublicNetworkConfiguration>>,
    dynamic: MailmanagerIngressPoint_NetworkConfigurationDynamic,
}

impl MailmanagerIngressPoint_NetworkConfiguration {
    #[doc = "Set the field `private_network_configuration`.\n"]
    pub fn set_private_network_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MailmanagerIngressPoint_NetworkConfiguration_PrivateNetworkConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.private_network_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.private_network_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `public_network_configuration`.\n"]
    pub fn set_public_network_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MailmanagerIngressPoint_NetworkConfiguration_PublicNetworkConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.public_network_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.public_network_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerIngressPoint_NetworkConfiguration {
    type O = terrars::BlockAssignable<MailmanagerIngressPoint_NetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerIngressPoint_NetworkConfiguration {}

impl BuildMailmanagerIngressPoint_NetworkConfiguration {
    pub fn build(self) -> MailmanagerIngressPoint_NetworkConfiguration {
        MailmanagerIngressPoint_NetworkConfiguration {
            private_network_configuration: core::default::Default::default(),
            public_network_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerIngressPoint_NetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerIngressPoint_NetworkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerIngressPoint_NetworkConfigurationRef {
        MailmanagerIngressPoint_NetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerIngressPoint_NetworkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `private_network_configuration` after provisioning.\n"]
    pub fn private_network_configuration(
        &self,
    ) -> terrars::ListRef<MailmanagerIngressPoint_NetworkConfiguration_PrivateNetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_network_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `public_network_configuration` after provisioning.\n"]
    pub fn public_network_configuration(
        &self,
    ) -> terrars::ListRef<MailmanagerIngressPoint_NetworkConfiguration_PublicNetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_network_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct MailmanagerIngressPoint_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MailmanagerIngressPoint_Timeouts {
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

impl terrars::ToListMappable for MailmanagerIngressPoint_Timeouts {
    type O = terrars::BlockAssignable<MailmanagerIngressPoint_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerIngressPoint_Timeouts {}

impl BuildMailmanagerIngressPoint_Timeouts {
    pub fn build(self) -> MailmanagerIngressPoint_Timeouts {
        MailmanagerIngressPoint_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerIngressPoint_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerIngressPoint_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MailmanagerIngressPoint_TimeoutsRef {
        MailmanagerIngressPoint_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerIngressPoint_TimeoutsRef {
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
