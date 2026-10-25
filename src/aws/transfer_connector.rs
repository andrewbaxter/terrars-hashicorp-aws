#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct TransferConnectorData_ {
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
    access_role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_policy_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    as2_config: std::option::Option<std::vec::Vec<TransferConnector_As2Config>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_config: std::option::Option<std::vec::Vec<TransferConnector_EgressConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sftp_config: std::option::Option<std::vec::Vec<TransferConnector_SftpConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<TransferConnector_Timeouts>,
    dynamic: TransferConnectorDynamic,
}

struct TransferConnector_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<TransferConnectorData_>,
}

#[derive(Clone)]
pub struct TransferConnector(std::rc::Rc<TransferConnector_>);

impl TransferConnector {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `logging_role`.\n"]
    pub fn set_logging_role(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().logging_role = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_policy_name`.\n"]
    pub fn set_security_policy_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().security_policy_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `as2_config`.\n"]
    pub fn set_as2_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferConnector_As2Config>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().as2_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.as2_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `egress_config`.\n"]
    pub fn set_egress_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferConnector_EgressConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().egress_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.egress_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sftp_config`.\n"]
    pub fn set_sftp_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferConnector_SftpConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sftp_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sftp_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<TransferConnector_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `access_role` after provisioning.\n"]
    pub fn access_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connector_id` after provisioning.\n"]
    pub fn connector_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_role` after provisioning.\n"]
    pub fn logging_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging_role", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_policy_name` after provisioning.\n"]
    pub fn security_policy_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_policy_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `as2_config` after provisioning.\n"]
    pub fn as2_config(&self) -> terrars::ListRef<TransferConnector_As2ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.as2_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egress_config` after provisioning.\n"]
    pub fn egress_config(&self) -> terrars::ListRef<TransferConnector_EgressConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sftp_config` after provisioning.\n"]
    pub fn sftp_config(&self) -> terrars::ListRef<TransferConnector_SftpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sftp_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TransferConnector_TimeoutsRef {
        TransferConnector_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for TransferConnector {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for TransferConnector { }

impl terrars::ToListMappable for TransferConnector {
    type O = terrars::ListRef<TransferConnectorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for TransferConnector_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_transfer_connector".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildTransferConnector {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub access_role: terrars::PrimField<std::string::String>,
}

impl BuildTransferConnector {
    pub fn build(self, stack: &mut terrars::Stack) -> TransferConnector {
        let out = TransferConnector(std::rc::Rc::new(TransferConnector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(TransferConnectorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                access_role: self.access_role,
                id: core::default::Default::default(),
                logging_role: core::default::Default::default(),
                region: core::default::Default::default(),
                security_policy_name: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                url: core::default::Default::default(),
                as2_config: core::default::Default::default(),
                egress_config: core::default::Default::default(),
                sftp_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct TransferConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferConnectorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl TransferConnectorRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_role` after provisioning.\n"]
    pub fn access_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connector_id` after provisioning.\n"]
    pub fn connector_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_role` after provisioning.\n"]
    pub fn logging_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.logging_role", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_policy_name` after provisioning.\n"]
    pub fn security_policy_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_policy_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `as2_config` after provisioning.\n"]
    pub fn as2_config(&self) -> terrars::ListRef<TransferConnector_As2ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.as2_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egress_config` after provisioning.\n"]
    pub fn egress_config(&self) -> terrars::ListRef<TransferConnector_EgressConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sftp_config` after provisioning.\n"]
    pub fn sftp_config(&self) -> terrars::ListRef<TransferConnector_SftpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sftp_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> TransferConnector_TimeoutsRef {
        TransferConnector_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct TransferConnectorDynamic {
    as2_config: std::option::Option<terrars::DynamicBlock<TransferConnector_As2Config>>,
    egress_config: std::option::Option<terrars::DynamicBlock<TransferConnector_EgressConfig>>,
    sftp_config: std::option::Option<terrars::DynamicBlock<TransferConnector_SftpConfig>>,
}

#[derive(Serialize)]
pub struct TransferConnector_As2Config {
    compression: terrars::PrimField<std::string::String>,
    encryption_algorithm: terrars::PrimField<std::string::String>,
    local_profile_id: terrars::PrimField<std::string::String>,
    mdn_response: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mdn_signing_algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_subject: std::option::Option<terrars::PrimField<std::string::String>>,
    partner_profile_id: terrars::PrimField<std::string::String>,
    signing_algorithm: terrars::PrimField<std::string::String>,
}

impl TransferConnector_As2Config {
    #[doc = "Set the field `mdn_signing_algorithm`.\n"]
    pub fn set_mdn_signing_algorithm(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.mdn_signing_algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message_subject`.\n"]
    pub fn set_message_subject(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message_subject = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TransferConnector_As2Config {
    type O = terrars::BlockAssignable<TransferConnector_As2Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferConnector_As2Config {
    #[doc = ""]
    pub compression: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub encryption_algorithm: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub local_profile_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub mdn_response: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub partner_profile_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub signing_algorithm: terrars::PrimField<std::string::String>,
}

impl BuildTransferConnector_As2Config {
    pub fn build(self) -> TransferConnector_As2Config {
        TransferConnector_As2Config {
            compression: self.compression,
            encryption_algorithm: self.encryption_algorithm,
            local_profile_id: self.local_profile_id,
            mdn_response: self.mdn_response,
            mdn_signing_algorithm: core::default::Default::default(),
            message_subject: core::default::Default::default(),
            partner_profile_id: self.partner_profile_id,
            signing_algorithm: self.signing_algorithm,
        }
    }
}

#[allow(dead_code)]
pub struct TransferConnector_As2ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferConnector_As2ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferConnector_As2ConfigRef {
        TransferConnector_As2ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferConnector_As2ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compression` after provisioning.\n"]
    pub fn compression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compression", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_algorithm` after provisioning.\n"]
    pub fn encryption_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `local_profile_id` after provisioning.\n"]
    pub fn local_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_profile_id", self.base))
    }

    #[doc = "Get a reference to the value of field `mdn_response` after provisioning.\n"]
    pub fn mdn_response(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mdn_response", self.base))
    }

    #[doc = "Get a reference to the value of field `mdn_signing_algorithm` after provisioning.\n"]
    pub fn mdn_signing_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mdn_signing_algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `message_subject` after provisioning.\n"]
    pub fn message_subject(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message_subject", self.base))
    }

    #[doc = "Get a reference to the value of field `partner_profile_id` after provisioning.\n"]
    pub fn partner_profile_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partner_profile_id", self.base))
    }

    #[doc = "Get a reference to the value of field `signing_algorithm` after provisioning.\n"]
    pub fn signing_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signing_algorithm", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferConnector_EgressConfig_VpcLattice {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_number: std::option::Option<terrars::PrimField<f64>>,
    resource_configuration_arn: terrars::PrimField<std::string::String>,
}

impl TransferConnector_EgressConfig_VpcLattice {
    #[doc = "Set the field `port_number`.\n"]
    pub fn set_port_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TransferConnector_EgressConfig_VpcLattice {
    type O = terrars::BlockAssignable<TransferConnector_EgressConfig_VpcLattice>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferConnector_EgressConfig_VpcLattice {
    #[doc = ""]
    pub resource_configuration_arn: terrars::PrimField<std::string::String>,
}

impl BuildTransferConnector_EgressConfig_VpcLattice {
    pub fn build(self) -> TransferConnector_EgressConfig_VpcLattice {
        TransferConnector_EgressConfig_VpcLattice {
            port_number: core::default::Default::default(),
            resource_configuration_arn: self.resource_configuration_arn,
        }
    }
}

#[allow(dead_code)]
pub struct TransferConnector_EgressConfig_VpcLatticeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferConnector_EgressConfig_VpcLatticeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferConnector_EgressConfig_VpcLatticeRef {
        TransferConnector_EgressConfig_VpcLatticeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferConnector_EgressConfig_VpcLatticeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port_number` after provisioning.\n"]
    pub fn port_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port_number", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_configuration_arn` after provisioning.\n"]
    pub fn resource_configuration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_configuration_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct TransferConnector_EgressConfigDynamic {
    vpc_lattice: std::option::Option<terrars::DynamicBlock<TransferConnector_EgressConfig_VpcLattice>>,
}

#[derive(Serialize)]
pub struct TransferConnector_EgressConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_lattice: std::option::Option<std::vec::Vec<TransferConnector_EgressConfig_VpcLattice>>,
    dynamic: TransferConnector_EgressConfigDynamic,
}

impl TransferConnector_EgressConfig {
    #[doc = "Set the field `vpc_lattice`.\n"]
    pub fn set_vpc_lattice(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<TransferConnector_EgressConfig_VpcLattice>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_lattice = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_lattice = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for TransferConnector_EgressConfig {
    type O = terrars::BlockAssignable<TransferConnector_EgressConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferConnector_EgressConfig {}

impl BuildTransferConnector_EgressConfig {
    pub fn build(self) -> TransferConnector_EgressConfig {
        TransferConnector_EgressConfig {
            vpc_lattice: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TransferConnector_EgressConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferConnector_EgressConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferConnector_EgressConfigRef {
        TransferConnector_EgressConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferConnector_EgressConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vpc_lattice` after provisioning.\n"]
    pub fn vpc_lattice(&self) -> terrars::ListRef<TransferConnector_EgressConfig_VpcLatticeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_lattice", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferConnector_SftpConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trusted_host_keys: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_secret_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TransferConnector_SftpConfig {
    #[doc = "Set the field `trusted_host_keys`.\n"]
    pub fn set_trusted_host_keys(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.trusted_host_keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_secret_id`.\n"]
    pub fn set_user_secret_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.user_secret_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TransferConnector_SftpConfig {
    type O = terrars::BlockAssignable<TransferConnector_SftpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferConnector_SftpConfig {}

impl BuildTransferConnector_SftpConfig {
    pub fn build(self) -> TransferConnector_SftpConfig {
        TransferConnector_SftpConfig {
            trusted_host_keys: core::default::Default::default(),
            user_secret_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TransferConnector_SftpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferConnector_SftpConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferConnector_SftpConfigRef {
        TransferConnector_SftpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferConnector_SftpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `trusted_host_keys` after provisioning.\n"]
    pub fn trusted_host_keys(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.trusted_host_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `user_secret_id` after provisioning.\n"]
    pub fn user_secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_secret_id", self.base))
    }
}

#[derive(Serialize)]
pub struct TransferConnector_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl TransferConnector_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for TransferConnector_Timeouts {
    type O = terrars::BlockAssignable<TransferConnector_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildTransferConnector_Timeouts {}

impl BuildTransferConnector_Timeouts {
    pub fn build(self) -> TransferConnector_Timeouts {
        TransferConnector_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct TransferConnector_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for TransferConnector_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> TransferConnector_TimeoutsRef {
        TransferConnector_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl TransferConnector_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
