#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataTransferConnectorData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataTransferConnector_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataTransferConnectorData_>,
}

#[derive(Clone)]
pub struct DataTransferConnector(std::rc::Rc<DataTransferConnector_>);

impl DataTransferConnector {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Get a reference to the value of field `access_role` after provisioning.\n"]
    pub fn access_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `as2_config` after provisioning.\n"]
    pub fn as2_config(&self) -> terrars::ListRef<DataTransferConnector_As2ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.as2_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egress_config` after provisioning.\n"]
    pub fn egress_config(&self) -> terrars::ListRef<DataTransferConnector_EgressConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_config", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `service_managed_egress_ip_addresses` after provisioning.\n"]
    pub fn service_managed_egress_ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.service_managed_egress_ip_addresses", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sftp_config` after provisioning.\n"]
    pub fn sftp_config(&self) -> terrars::ListRef<DataTransferConnector_SftpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sftp_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}

impl terrars::Referable for DataTransferConnector {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataTransferConnector { }

impl terrars::ToListMappable for DataTransferConnector {
    type O = terrars::ListRef<DataTransferConnectorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataTransferConnector_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_transfer_connector".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataTransferConnector {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildDataTransferConnector {
    pub fn build(self, stack: &mut terrars::Stack) -> DataTransferConnector {
        let out = DataTransferConnector(std::rc::Rc::new(DataTransferConnector_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataTransferConnectorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: self.id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataTransferConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTransferConnectorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataTransferConnectorRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `access_role` after provisioning.\n"]
    pub fn access_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `as2_config` after provisioning.\n"]
    pub fn as2_config(&self) -> terrars::ListRef<DataTransferConnector_As2ConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.as2_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `egress_config` after provisioning.\n"]
    pub fn egress_config(&self) -> terrars::ListRef<DataTransferConnector_EgressConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_config", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `service_managed_egress_ip_addresses` after provisioning.\n"]
    pub fn service_managed_egress_ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.service_managed_egress_ip_addresses", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sftp_config` after provisioning.\n"]
    pub fn sftp_config(&self) -> terrars::ListRef<DataTransferConnector_SftpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sftp_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataTransferConnector_As2Config {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    basic_auth_secret_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_profile_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mdn_response: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mdn_signing_algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message_subject: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partner_profile_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    singing_algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataTransferConnector_As2Config {
    #[doc = "Set the field `basic_auth_secret_id`.\n"]
    pub fn set_basic_auth_secret_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.basic_auth_secret_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compression`.\n"]
    pub fn set_compression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_algorithm`.\n"]
    pub fn set_encryption_algorithm(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.encryption_algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_profile_id`.\n"]
    pub fn set_local_profile_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_profile_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mdn_response`.\n"]
    pub fn set_mdn_response(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mdn_response = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `partner_profile_id`.\n"]
    pub fn set_partner_profile_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.partner_profile_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `singing_algorithm`.\n"]
    pub fn set_singing_algorithm(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.singing_algorithm = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataTransferConnector_As2Config {
    type O = terrars::BlockAssignable<DataTransferConnector_As2Config>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTransferConnector_As2Config {}

impl BuildDataTransferConnector_As2Config {
    pub fn build(self) -> DataTransferConnector_As2Config {
        DataTransferConnector_As2Config {
            basic_auth_secret_id: core::default::Default::default(),
            compression: core::default::Default::default(),
            encryption_algorithm: core::default::Default::default(),
            local_profile_id: core::default::Default::default(),
            mdn_response: core::default::Default::default(),
            mdn_signing_algorithm: core::default::Default::default(),
            message_subject: core::default::Default::default(),
            partner_profile_id: core::default::Default::default(),
            singing_algorithm: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataTransferConnector_As2ConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTransferConnector_As2ConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataTransferConnector_As2ConfigRef {
        DataTransferConnector_As2ConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTransferConnector_As2ConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `basic_auth_secret_id` after provisioning.\n"]
    pub fn basic_auth_secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.basic_auth_secret_id", self.base))
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

    #[doc = "Get a reference to the value of field `singing_algorithm` after provisioning.\n"]
    pub fn singing_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.singing_algorithm", self.base))
    }
}

#[derive(Serialize)]
pub struct DataTransferConnector_EgressConfig_VpcLattice {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_number: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_configuration_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataTransferConnector_EgressConfig_VpcLattice {
    #[doc = "Set the field `port_number`.\n"]
    pub fn set_port_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_configuration_arn`.\n"]
    pub fn set_resource_configuration_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.resource_configuration_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataTransferConnector_EgressConfig_VpcLattice {
    type O = terrars::BlockAssignable<DataTransferConnector_EgressConfig_VpcLattice>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTransferConnector_EgressConfig_VpcLattice {}

impl BuildDataTransferConnector_EgressConfig_VpcLattice {
    pub fn build(self) -> DataTransferConnector_EgressConfig_VpcLattice {
        DataTransferConnector_EgressConfig_VpcLattice {
            port_number: core::default::Default::default(),
            resource_configuration_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataTransferConnector_EgressConfig_VpcLatticeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTransferConnector_EgressConfig_VpcLatticeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataTransferConnector_EgressConfig_VpcLatticeRef {
        DataTransferConnector_EgressConfig_VpcLatticeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTransferConnector_EgressConfig_VpcLatticeRef {
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

#[derive(Serialize)]
pub struct DataTransferConnector_EgressConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_lattice: std::option::Option<terrars::ListField<DataTransferConnector_EgressConfig_VpcLattice>>,
}

impl DataTransferConnector_EgressConfig {
    #[doc = "Set the field `vpc_lattice`.\n"]
    pub fn set_vpc_lattice(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataTransferConnector_EgressConfig_VpcLattice>>,
    ) -> Self {
        self.vpc_lattice = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataTransferConnector_EgressConfig {
    type O = terrars::BlockAssignable<DataTransferConnector_EgressConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTransferConnector_EgressConfig {}

impl BuildDataTransferConnector_EgressConfig {
    pub fn build(self) -> DataTransferConnector_EgressConfig {
        DataTransferConnector_EgressConfig { vpc_lattice: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataTransferConnector_EgressConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTransferConnector_EgressConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataTransferConnector_EgressConfigRef {
        DataTransferConnector_EgressConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTransferConnector_EgressConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vpc_lattice` after provisioning.\n"]
    pub fn vpc_lattice(&self) -> terrars::ListRef<DataTransferConnector_EgressConfig_VpcLatticeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_lattice", self.base))
    }
}

#[derive(Serialize)]
pub struct DataTransferConnector_SftpConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trusted_host_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_secret_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataTransferConnector_SftpConfig {
    #[doc = "Set the field `trusted_host_keys`.\n"]
    pub fn set_trusted_host_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
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

impl terrars::ToListMappable for DataTransferConnector_SftpConfig {
    type O = terrars::BlockAssignable<DataTransferConnector_SftpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataTransferConnector_SftpConfig {}

impl BuildDataTransferConnector_SftpConfig {
    pub fn build(self) -> DataTransferConnector_SftpConfig {
        DataTransferConnector_SftpConfig {
            trusted_host_keys: core::default::Default::default(),
            user_secret_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataTransferConnector_SftpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataTransferConnector_SftpConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataTransferConnector_SftpConfigRef {
        DataTransferConnector_SftpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataTransferConnector_SftpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `trusted_host_keys` after provisioning.\n"]
    pub fn trusted_host_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.trusted_host_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `user_secret_id` after provisioning.\n"]
    pub fn user_secret_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_secret_id", self.base))
    }
}
