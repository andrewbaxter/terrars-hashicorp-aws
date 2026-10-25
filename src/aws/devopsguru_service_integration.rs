#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DevopsguruServiceIntegrationData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_server_side_encryption: std::option::Option<std::vec::Vec<DevopsguruServiceIntegration_KmsServerSideEncryption>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logs_anomaly_detection: std::option::Option<std::vec::Vec<DevopsguruServiceIntegration_LogsAnomalyDetection>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ops_center: std::option::Option<std::vec::Vec<DevopsguruServiceIntegration_OpsCenter>>,
    dynamic: DevopsguruServiceIntegrationDynamic,
}

struct DevopsguruServiceIntegration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DevopsguruServiceIntegrationData_>,
}

#[derive(Clone)]
pub struct DevopsguruServiceIntegration(std::rc::Rc<DevopsguruServiceIntegration_>);

impl DevopsguruServiceIntegration {
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

    #[doc = "Set the field `kms_server_side_encryption`.\n"]
    pub fn set_kms_server_side_encryption(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DevopsguruServiceIntegration_KmsServerSideEncryption>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kms_server_side_encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kms_server_side_encryption = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logs_anomaly_detection`.\n"]
    pub fn set_logs_anomaly_detection(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DevopsguruServiceIntegration_LogsAnomalyDetection>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logs_anomaly_detection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logs_anomaly_detection = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ops_center`.\n"]
    pub fn set_ops_center(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DevopsguruServiceIntegration_OpsCenter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ops_center = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ops_center = std::option::Option::Some(d);
            },
        }
        self
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

    #[doc = "Get a reference to the value of field `kms_server_side_encryption` after provisioning.\n"]
    pub fn kms_server_side_encryption(
        &self,
    ) -> terrars::ListRef<DevopsguruServiceIntegration_KmsServerSideEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kms_server_side_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logs_anomaly_detection` after provisioning.\n"]
    pub fn logs_anomaly_detection(&self) -> terrars::ListRef<DevopsguruServiceIntegration_LogsAnomalyDetectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logs_anomaly_detection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ops_center` after provisioning.\n"]
    pub fn ops_center(&self) -> terrars::ListRef<DevopsguruServiceIntegration_OpsCenterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ops_center", self.extract_ref()))
    }
}

impl terrars::Referable for DevopsguruServiceIntegration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DevopsguruServiceIntegration { }

impl terrars::ToListMappable for DevopsguruServiceIntegration {
    type O = terrars::ListRef<DevopsguruServiceIntegrationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DevopsguruServiceIntegration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_devopsguru_service_integration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDevopsguruServiceIntegration {
    pub tf_id: std::string::String,
}

impl BuildDevopsguruServiceIntegration {
    pub fn build(self, stack: &mut terrars::Stack) -> DevopsguruServiceIntegration {
        let out = DevopsguruServiceIntegration(std::rc::Rc::new(DevopsguruServiceIntegration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DevopsguruServiceIntegrationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                region: core::default::Default::default(),
                kms_server_side_encryption: core::default::Default::default(),
                logs_anomaly_detection: core::default::Default::default(),
                ops_center: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DevopsguruServiceIntegrationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DevopsguruServiceIntegrationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DevopsguruServiceIntegrationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `kms_server_side_encryption` after provisioning.\n"]
    pub fn kms_server_side_encryption(
        &self,
    ) -> terrars::ListRef<DevopsguruServiceIntegration_KmsServerSideEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kms_server_side_encryption", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logs_anomaly_detection` after provisioning.\n"]
    pub fn logs_anomaly_detection(&self) -> terrars::ListRef<DevopsguruServiceIntegration_LogsAnomalyDetectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logs_anomaly_detection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ops_center` after provisioning.\n"]
    pub fn ops_center(&self) -> terrars::ListRef<DevopsguruServiceIntegration_OpsCenterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ops_center", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DevopsguruServiceIntegrationDynamic {
    kms_server_side_encryption: std
    ::option
    ::Option<terrars::DynamicBlock<DevopsguruServiceIntegration_KmsServerSideEncryption>>,
    logs_anomaly_detection: std
    ::option
    ::Option<terrars::DynamicBlock<DevopsguruServiceIntegration_LogsAnomalyDetection>>,
    ops_center: std::option::Option<terrars::DynamicBlock<DevopsguruServiceIntegration_OpsCenter>>,
}

#[derive(Serialize)]
pub struct DevopsguruServiceIntegration_KmsServerSideEncryption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opt_in_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DevopsguruServiceIntegration_KmsServerSideEncryption {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `opt_in_status`.\n"]
    pub fn set_opt_in_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.opt_in_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DevopsguruServiceIntegration_KmsServerSideEncryption {
    type O = terrars::BlockAssignable<DevopsguruServiceIntegration_KmsServerSideEncryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevopsguruServiceIntegration_KmsServerSideEncryption {}

impl BuildDevopsguruServiceIntegration_KmsServerSideEncryption {
    pub fn build(self) -> DevopsguruServiceIntegration_KmsServerSideEncryption {
        DevopsguruServiceIntegration_KmsServerSideEncryption {
            kms_key_id: core::default::Default::default(),
            opt_in_status: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DevopsguruServiceIntegration_KmsServerSideEncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DevopsguruServiceIntegration_KmsServerSideEncryptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DevopsguruServiceIntegration_KmsServerSideEncryptionRef {
        DevopsguruServiceIntegration_KmsServerSideEncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevopsguruServiceIntegration_KmsServerSideEncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `opt_in_status` after provisioning.\n"]
    pub fn opt_in_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.opt_in_status", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DevopsguruServiceIntegration_LogsAnomalyDetection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opt_in_status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DevopsguruServiceIntegration_LogsAnomalyDetection {
    #[doc = "Set the field `opt_in_status`.\n"]
    pub fn set_opt_in_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.opt_in_status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DevopsguruServiceIntegration_LogsAnomalyDetection {
    type O = terrars::BlockAssignable<DevopsguruServiceIntegration_LogsAnomalyDetection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevopsguruServiceIntegration_LogsAnomalyDetection {}

impl BuildDevopsguruServiceIntegration_LogsAnomalyDetection {
    pub fn build(self) -> DevopsguruServiceIntegration_LogsAnomalyDetection {
        DevopsguruServiceIntegration_LogsAnomalyDetection { opt_in_status: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DevopsguruServiceIntegration_LogsAnomalyDetectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DevopsguruServiceIntegration_LogsAnomalyDetectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DevopsguruServiceIntegration_LogsAnomalyDetectionRef {
        DevopsguruServiceIntegration_LogsAnomalyDetectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevopsguruServiceIntegration_LogsAnomalyDetectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `opt_in_status` after provisioning.\n"]
    pub fn opt_in_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.opt_in_status", self.base))
    }
}

#[derive(Serialize)]
pub struct DevopsguruServiceIntegration_OpsCenter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opt_in_status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DevopsguruServiceIntegration_OpsCenter {
    #[doc = "Set the field `opt_in_status`.\n"]
    pub fn set_opt_in_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.opt_in_status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DevopsguruServiceIntegration_OpsCenter {
    type O = terrars::BlockAssignable<DevopsguruServiceIntegration_OpsCenter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDevopsguruServiceIntegration_OpsCenter {}

impl BuildDevopsguruServiceIntegration_OpsCenter {
    pub fn build(self) -> DevopsguruServiceIntegration_OpsCenter {
        DevopsguruServiceIntegration_OpsCenter { opt_in_status: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DevopsguruServiceIntegration_OpsCenterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DevopsguruServiceIntegration_OpsCenterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DevopsguruServiceIntegration_OpsCenterRef {
        DevopsguruServiceIntegration_OpsCenterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DevopsguruServiceIntegration_OpsCenterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `opt_in_status` after provisioning.\n"]
    pub fn opt_in_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.opt_in_status", self.base))
    }
}
