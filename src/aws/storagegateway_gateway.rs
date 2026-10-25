#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct StoragegatewayGatewayData_ {
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
    activation_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    average_download_rate_limit_in_bits_per_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    average_upload_rate_limit_in_bits_per_sec: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_log_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    gateway_name: terrars::PrimField<std::string::String>,
    gateway_timezone: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway_vpc_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    medium_changer_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    smb_file_share_visibility: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    smb_guest_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    smb_security_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tape_drive_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_start_time: std::option::Option<std::vec::Vec<StoragegatewayGateway_MaintenanceStartTime>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    smb_active_directory_settings: std::option::Option<std::vec::Vec<StoragegatewayGateway_SmbActiveDirectorySettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<StoragegatewayGateway_Timeouts>,
    dynamic: StoragegatewayGatewayDynamic,
}

struct StoragegatewayGateway_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<StoragegatewayGatewayData_>,
}

#[derive(Clone)]
pub struct StoragegatewayGateway(std::rc::Rc<StoragegatewayGateway_>);

impl StoragegatewayGateway {
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

    #[doc = "Set the field `activation_key`.\n"]
    pub fn set_activation_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().activation_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `average_download_rate_limit_in_bits_per_sec`.\n"]
    pub fn set_average_download_rate_limit_in_bits_per_sec(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().average_download_rate_limit_in_bits_per_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `average_upload_rate_limit_in_bits_per_sec`.\n"]
    pub fn set_average_upload_rate_limit_in_bits_per_sec(
        self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.0.data.borrow_mut().average_upload_rate_limit_in_bits_per_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_log_group_arn`.\n"]
    pub fn set_cloudwatch_log_group_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().cloudwatch_log_group_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_ip_address`.\n"]
    pub fn set_gateway_ip_address(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().gateway_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_type`.\n"]
    pub fn set_gateway_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().gateway_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_vpc_endpoint`.\n"]
    pub fn set_gateway_vpc_endpoint(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().gateway_vpc_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `medium_changer_type`.\n"]
    pub fn set_medium_changer_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().medium_changer_type = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `smb_file_share_visibility`.\n"]
    pub fn set_smb_file_share_visibility(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().smb_file_share_visibility = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `smb_guest_password`.\n"]
    pub fn set_smb_guest_password(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().smb_guest_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `smb_security_strategy`.\n"]
    pub fn set_smb_security_strategy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().smb_security_strategy = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tape_drive_type`.\n"]
    pub fn set_tape_drive_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tape_drive_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maintenance_start_time`.\n"]
    pub fn set_maintenance_start_time(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StoragegatewayGateway_MaintenanceStartTime>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_start_time = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_start_time = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `smb_active_directory_settings`.\n"]
    pub fn set_smb_active_directory_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StoragegatewayGateway_SmbActiveDirectorySettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().smb_active_directory_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.smb_active_directory_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<StoragegatewayGateway_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `activation_key` after provisioning.\n"]
    pub fn activation_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activation_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `average_download_rate_limit_in_bits_per_sec` after provisioning.\n"]
    pub fn average_download_rate_limit_in_bits_per_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.average_download_rate_limit_in_bits_per_sec", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `average_upload_rate_limit_in_bits_per_sec` after provisioning.\n"]
    pub fn average_upload_rate_limit_in_bits_per_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.average_upload_rate_limit_in_bits_per_sec", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cloudwatch_log_group_arn` after provisioning.\n"]
    pub fn cloudwatch_log_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_log_group_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ec2_instance_id` after provisioning.\n"]
    pub fn ec2_instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ec2_instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_ip_address` after provisioning.\n"]
    pub fn gateway_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_ip_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_name` after provisioning.\n"]
    pub fn gateway_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_network_interface` after provisioning.\n"]
    pub fn gateway_network_interface(&self) -> terrars::ListRef<StoragegatewayGateway_GatewayNetworkInterfaceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gateway_network_interface", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_timezone` after provisioning.\n"]
    pub fn gateway_timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_type` after provisioning.\n"]
    pub fn gateway_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_vpc_endpoint` after provisioning.\n"]
    pub fn gateway_vpc_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_vpc_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_environment` after provisioning.\n"]
    pub fn host_environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `medium_changer_type` after provisioning.\n"]
    pub fn medium_changer_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.medium_changer_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `smb_file_share_visibility` after provisioning.\n"]
    pub fn smb_file_share_visibility(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.smb_file_share_visibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `smb_guest_password` after provisioning.\n"]
    pub fn smb_guest_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.smb_guest_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `smb_security_strategy` after provisioning.\n"]
    pub fn smb_security_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.smb_security_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tape_drive_type` after provisioning.\n"]
    pub fn tape_drive_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tape_drive_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_start_time` after provisioning.\n"]
    pub fn maintenance_start_time(&self) -> terrars::ListRef<StoragegatewayGateway_MaintenanceStartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `smb_active_directory_settings` after provisioning.\n"]
    pub fn smb_active_directory_settings(&self) -> terrars::ListRef<StoragegatewayGateway_SmbActiveDirectorySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.smb_active_directory_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StoragegatewayGateway_TimeoutsRef {
        StoragegatewayGateway_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for StoragegatewayGateway {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for StoragegatewayGateway { }

impl terrars::ToListMappable for StoragegatewayGateway {
    type O = terrars::ListRef<StoragegatewayGatewayRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for StoragegatewayGateway_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_storagegateway_gateway".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStoragegatewayGateway {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub gateway_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub gateway_timezone: terrars::PrimField<std::string::String>,
}

impl BuildStoragegatewayGateway {
    pub fn build(self, stack: &mut terrars::Stack) -> StoragegatewayGateway {
        let out = StoragegatewayGateway(std::rc::Rc::new(StoragegatewayGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(StoragegatewayGatewayData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                activation_key: core::default::Default::default(),
                average_download_rate_limit_in_bits_per_sec: core::default::Default::default(),
                average_upload_rate_limit_in_bits_per_sec: core::default::Default::default(),
                cloudwatch_log_group_arn: core::default::Default::default(),
                gateway_ip_address: core::default::Default::default(),
                gateway_name: self.gateway_name,
                gateway_timezone: self.gateway_timezone,
                gateway_type: core::default::Default::default(),
                gateway_vpc_endpoint: core::default::Default::default(),
                id: core::default::Default::default(),
                medium_changer_type: core::default::Default::default(),
                region: core::default::Default::default(),
                smb_file_share_visibility: core::default::Default::default(),
                smb_guest_password: core::default::Default::default(),
                smb_security_strategy: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                tape_drive_type: core::default::Default::default(),
                maintenance_start_time: core::default::Default::default(),
                smb_active_directory_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StoragegatewayGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StoragegatewayGatewayRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl StoragegatewayGatewayRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `activation_key` after provisioning.\n"]
    pub fn activation_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.activation_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `average_download_rate_limit_in_bits_per_sec` after provisioning.\n"]
    pub fn average_download_rate_limit_in_bits_per_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.average_download_rate_limit_in_bits_per_sec", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `average_upload_rate_limit_in_bits_per_sec` after provisioning.\n"]
    pub fn average_upload_rate_limit_in_bits_per_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.average_upload_rate_limit_in_bits_per_sec", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cloudwatch_log_group_arn` after provisioning.\n"]
    pub fn cloudwatch_log_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_log_group_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ec2_instance_id` after provisioning.\n"]
    pub fn ec2_instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ec2_instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_ip_address` after provisioning.\n"]
    pub fn gateway_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_ip_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_name` after provisioning.\n"]
    pub fn gateway_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_network_interface` after provisioning.\n"]
    pub fn gateway_network_interface(&self) -> terrars::ListRef<StoragegatewayGateway_GatewayNetworkInterfaceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.gateway_network_interface", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_timezone` after provisioning.\n"]
    pub fn gateway_timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_timezone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_type` after provisioning.\n"]
    pub fn gateway_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_vpc_endpoint` after provisioning.\n"]
    pub fn gateway_vpc_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_vpc_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_environment` after provisioning.\n"]
    pub fn host_environment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_environment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `medium_changer_type` after provisioning.\n"]
    pub fn medium_changer_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.medium_changer_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `smb_file_share_visibility` after provisioning.\n"]
    pub fn smb_file_share_visibility(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.smb_file_share_visibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `smb_guest_password` after provisioning.\n"]
    pub fn smb_guest_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.smb_guest_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `smb_security_strategy` after provisioning.\n"]
    pub fn smb_security_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.smb_security_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tape_drive_type` after provisioning.\n"]
    pub fn tape_drive_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tape_drive_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_start_time` after provisioning.\n"]
    pub fn maintenance_start_time(&self) -> terrars::ListRef<StoragegatewayGateway_MaintenanceStartTimeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `smb_active_directory_settings` after provisioning.\n"]
    pub fn smb_active_directory_settings(&self) -> terrars::ListRef<StoragegatewayGateway_SmbActiveDirectorySettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.smb_active_directory_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StoragegatewayGateway_TimeoutsRef {
        StoragegatewayGateway_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct StoragegatewayGatewayDynamic {
    maintenance_start_time: std::option::Option<terrars::DynamicBlock<StoragegatewayGateway_MaintenanceStartTime>>,
    smb_active_directory_settings: std
    ::option
    ::Option<terrars::DynamicBlock<StoragegatewayGateway_SmbActiveDirectorySettings>>,
}

#[derive(Serialize)]
pub struct StoragegatewayGateway_GatewayNetworkInterface {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_address: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StoragegatewayGateway_GatewayNetworkInterface {
    #[doc = "Set the field `ipv4_address`.\n"]
    pub fn set_ipv4_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ipv4_address = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StoragegatewayGateway_GatewayNetworkInterface {
    type O = terrars::BlockAssignable<StoragegatewayGateway_GatewayNetworkInterface>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStoragegatewayGateway_GatewayNetworkInterface {}

impl BuildStoragegatewayGateway_GatewayNetworkInterface {
    pub fn build(self) -> StoragegatewayGateway_GatewayNetworkInterface {
        StoragegatewayGateway_GatewayNetworkInterface { ipv4_address: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct StoragegatewayGateway_GatewayNetworkInterfaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StoragegatewayGateway_GatewayNetworkInterfaceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StoragegatewayGateway_GatewayNetworkInterfaceRef {
        StoragegatewayGateway_GatewayNetworkInterfaceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StoragegatewayGateway_GatewayNetworkInterfaceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ipv4_address` after provisioning.\n"]
    pub fn ipv4_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address", self.base))
    }
}

#[derive(Serialize)]
pub struct StoragegatewayGateway_MaintenanceStartTime {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_of_month: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    day_of_week: std::option::Option<terrars::PrimField<std::string::String>>,
    hour_of_day: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minute_of_hour: std::option::Option<terrars::PrimField<f64>>,
}

impl StoragegatewayGateway_MaintenanceStartTime {
    #[doc = "Set the field `day_of_month`.\n"]
    pub fn set_day_of_month(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.day_of_month = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `day_of_week`.\n"]
    pub fn set_day_of_week(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.day_of_week = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minute_of_hour`.\n"]
    pub fn set_minute_of_hour(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minute_of_hour = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StoragegatewayGateway_MaintenanceStartTime {
    type O = terrars::BlockAssignable<StoragegatewayGateway_MaintenanceStartTime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStoragegatewayGateway_MaintenanceStartTime {
    #[doc = ""]
    pub hour_of_day: terrars::PrimField<f64>,
}

impl BuildStoragegatewayGateway_MaintenanceStartTime {
    pub fn build(self) -> StoragegatewayGateway_MaintenanceStartTime {
        StoragegatewayGateway_MaintenanceStartTime {
            day_of_month: core::default::Default::default(),
            day_of_week: core::default::Default::default(),
            hour_of_day: self.hour_of_day,
            minute_of_hour: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StoragegatewayGateway_MaintenanceStartTimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StoragegatewayGateway_MaintenanceStartTimeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StoragegatewayGateway_MaintenanceStartTimeRef {
        StoragegatewayGateway_MaintenanceStartTimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StoragegatewayGateway_MaintenanceStartTimeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_month` after provisioning.\n"]
    pub fn day_of_month(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_month", self.base))
    }

    #[doc = "Get a reference to the value of field `day_of_week` after provisioning.\n"]
    pub fn day_of_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_week", self.base))
    }

    #[doc = "Get a reference to the value of field `hour_of_day` after provisioning.\n"]
    pub fn hour_of_day(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hour_of_day", self.base))
    }

    #[doc = "Get a reference to the value of field `minute_of_hour` after provisioning.\n"]
    pub fn minute_of_hour(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minute_of_hour", self.base))
    }
}

#[derive(Serialize)]
pub struct StoragegatewayGateway_SmbActiveDirectorySettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_controllers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organizational_unit: std::option::Option<terrars::PrimField<std::string::String>>,
    password: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    username: terrars::PrimField<std::string::String>,
}

impl StoragegatewayGateway_SmbActiveDirectorySettings {
    #[doc = "Set the field `domain_controllers`.\n"]
    pub fn set_domain_controllers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.domain_controllers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_unit`.\n"]
    pub fn set_organizational_unit(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.organizational_unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_in_seconds`.\n"]
    pub fn set_timeout_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StoragegatewayGateway_SmbActiveDirectorySettings {
    type O = terrars::BlockAssignable<StoragegatewayGateway_SmbActiveDirectorySettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStoragegatewayGateway_SmbActiveDirectorySettings {
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildStoragegatewayGateway_SmbActiveDirectorySettings {
    pub fn build(self) -> StoragegatewayGateway_SmbActiveDirectorySettings {
        StoragegatewayGateway_SmbActiveDirectorySettings {
            domain_controllers: core::default::Default::default(),
            domain_name: self.domain_name,
            organizational_unit: core::default::Default::default(),
            password: self.password,
            timeout_in_seconds: core::default::Default::default(),
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct StoragegatewayGateway_SmbActiveDirectorySettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StoragegatewayGateway_SmbActiveDirectorySettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StoragegatewayGateway_SmbActiveDirectorySettingsRef {
        StoragegatewayGateway_SmbActiveDirectorySettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StoragegatewayGateway_SmbActiveDirectorySettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active_directory_status` after provisioning.\n"]
    pub fn active_directory_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active_directory_status", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_controllers` after provisioning.\n"]
    pub fn domain_controllers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.domain_controllers", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `organizational_unit` after provisioning.\n"]
    pub fn organizational_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organizational_unit", self.base))
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_in_seconds` after provisioning.\n"]
    pub fn timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct StoragegatewayGateway_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StoragegatewayGateway_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StoragegatewayGateway_Timeouts {
    type O = terrars::BlockAssignable<StoragegatewayGateway_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStoragegatewayGateway_Timeouts {}

impl BuildStoragegatewayGateway_Timeouts {
    pub fn build(self) -> StoragegatewayGateway_Timeouts {
        StoragegatewayGateway_Timeouts { create: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct StoragegatewayGateway_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StoragegatewayGateway_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StoragegatewayGateway_TimeoutsRef {
        StoragegatewayGateway_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StoragegatewayGateway_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
