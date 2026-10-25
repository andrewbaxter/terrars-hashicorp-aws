#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FsxOntapStorageVirtualMachineData_ {
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
    file_system_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_volume_security_style: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    svm_admin_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    active_directory_configuration: std
    ::option
    ::Option<std::vec::Vec<FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FsxOntapStorageVirtualMachine_Timeouts>,
    dynamic: FsxOntapStorageVirtualMachineDynamic,
}

struct FsxOntapStorageVirtualMachine_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FsxOntapStorageVirtualMachineData_>,
}

#[derive(Clone)]
pub struct FsxOntapStorageVirtualMachine(std::rc::Rc<FsxOntapStorageVirtualMachine_>);

impl FsxOntapStorageVirtualMachine {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `root_volume_security_style`.\n"]
    pub fn set_root_volume_security_style(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().root_volume_security_style = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `svm_admin_password`.\n"]
    pub fn set_svm_admin_password(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().svm_admin_password = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `active_directory_configuration`.\n"]
    pub fn set_active_directory_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().active_directory_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.active_directory_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FsxOntapStorageVirtualMachine_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::ListRef<FsxOntapStorageVirtualMachine_EndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `root_volume_security_style` after provisioning.\n"]
    pub fn root_volume_security_style(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_volume_security_style", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subtype` after provisioning.\n"]
    pub fn subtype(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subtype", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `svm_admin_password` after provisioning.\n"]
    pub fn svm_admin_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.svm_admin_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `active_directory_configuration` after provisioning.\n"]
    pub fn active_directory_configuration(
        &self,
    ) -> terrars::ListRef<FsxOntapStorageVirtualMachine_ActiveDirectoryConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.active_directory_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOntapStorageVirtualMachine_TimeoutsRef {
        FsxOntapStorageVirtualMachine_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for FsxOntapStorageVirtualMachine {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FsxOntapStorageVirtualMachine { }

impl terrars::ToListMappable for FsxOntapStorageVirtualMachine {
    type O = terrars::ListRef<FsxOntapStorageVirtualMachineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FsxOntapStorageVirtualMachine_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_fsx_ontap_storage_virtual_machine".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFsxOntapStorageVirtualMachine {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub file_system_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildFsxOntapStorageVirtualMachine {
    pub fn build(self, stack: &mut terrars::Stack) -> FsxOntapStorageVirtualMachine {
        let out = FsxOntapStorageVirtualMachine(std::rc::Rc::new(FsxOntapStorageVirtualMachine_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FsxOntapStorageVirtualMachineData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                file_system_id: self.file_system_id,
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                root_volume_security_style: core::default::Default::default(),
                svm_admin_password: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                active_directory_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FsxOntapStorageVirtualMachineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapStorageVirtualMachineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FsxOntapStorageVirtualMachineRef {
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

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::ListRef<FsxOntapStorageVirtualMachine_EndpointsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.endpoints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `root_volume_security_style` after provisioning.\n"]
    pub fn root_volume_security_style(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_volume_security_style", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subtype` after provisioning.\n"]
    pub fn subtype(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subtype", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `svm_admin_password` after provisioning.\n"]
    pub fn svm_admin_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.svm_admin_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `active_directory_configuration` after provisioning.\n"]
    pub fn active_directory_configuration(
        &self,
    ) -> terrars::ListRef<FsxOntapStorageVirtualMachine_ActiveDirectoryConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.active_directory_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOntapStorageVirtualMachine_TimeoutsRef {
        FsxOntapStorageVirtualMachine_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct FsxOntapStorageVirtualMachineDynamic {
    active_directory_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration>>,
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachine_Endpoints_Iscsi {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl FsxOntapStorageVirtualMachine_Endpoints_Iscsi {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_addresses`.\n"]
    pub fn set_ip_addresses(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_addresses = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxOntapStorageVirtualMachine_Endpoints_Iscsi {
    type O = terrars::BlockAssignable<FsxOntapStorageVirtualMachine_Endpoints_Iscsi>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachine_Endpoints_Iscsi {}

impl BuildFsxOntapStorageVirtualMachine_Endpoints_Iscsi {
    pub fn build(self) -> FsxOntapStorageVirtualMachine_Endpoints_Iscsi {
        FsxOntapStorageVirtualMachine_Endpoints_Iscsi {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOntapStorageVirtualMachine_Endpoints_IscsiRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapStorageVirtualMachine_Endpoints_IscsiRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOntapStorageVirtualMachine_Endpoints_IscsiRef {
        FsxOntapStorageVirtualMachine_Endpoints_IscsiRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachine_Endpoints_IscsiRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachine_Endpoints_Management {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl FsxOntapStorageVirtualMachine_Endpoints_Management {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_addresses`.\n"]
    pub fn set_ip_addresses(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_addresses = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxOntapStorageVirtualMachine_Endpoints_Management {
    type O = terrars::BlockAssignable<FsxOntapStorageVirtualMachine_Endpoints_Management>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachine_Endpoints_Management {}

impl BuildFsxOntapStorageVirtualMachine_Endpoints_Management {
    pub fn build(self) -> FsxOntapStorageVirtualMachine_Endpoints_Management {
        FsxOntapStorageVirtualMachine_Endpoints_Management {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOntapStorageVirtualMachine_Endpoints_ManagementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapStorageVirtualMachine_Endpoints_ManagementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOntapStorageVirtualMachine_Endpoints_ManagementRef {
        FsxOntapStorageVirtualMachine_Endpoints_ManagementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachine_Endpoints_ManagementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachine_Endpoints_Nfs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl FsxOntapStorageVirtualMachine_Endpoints_Nfs {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_addresses`.\n"]
    pub fn set_ip_addresses(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_addresses = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxOntapStorageVirtualMachine_Endpoints_Nfs {
    type O = terrars::BlockAssignable<FsxOntapStorageVirtualMachine_Endpoints_Nfs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachine_Endpoints_Nfs {}

impl BuildFsxOntapStorageVirtualMachine_Endpoints_Nfs {
    pub fn build(self) -> FsxOntapStorageVirtualMachine_Endpoints_Nfs {
        FsxOntapStorageVirtualMachine_Endpoints_Nfs {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOntapStorageVirtualMachine_Endpoints_NfsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapStorageVirtualMachine_Endpoints_NfsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOntapStorageVirtualMachine_Endpoints_NfsRef {
        FsxOntapStorageVirtualMachine_Endpoints_NfsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachine_Endpoints_NfsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachine_Endpoints_Smb {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl FsxOntapStorageVirtualMachine_Endpoints_Smb {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_addresses`.\n"]
    pub fn set_ip_addresses(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_addresses = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxOntapStorageVirtualMachine_Endpoints_Smb {
    type O = terrars::BlockAssignable<FsxOntapStorageVirtualMachine_Endpoints_Smb>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachine_Endpoints_Smb {}

impl BuildFsxOntapStorageVirtualMachine_Endpoints_Smb {
    pub fn build(self) -> FsxOntapStorageVirtualMachine_Endpoints_Smb {
        FsxOntapStorageVirtualMachine_Endpoints_Smb {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOntapStorageVirtualMachine_Endpoints_SmbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapStorageVirtualMachine_Endpoints_SmbRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOntapStorageVirtualMachine_Endpoints_SmbRef {
        FsxOntapStorageVirtualMachine_Endpoints_SmbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachine_Endpoints_SmbRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachine_Endpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iscsi: std::option::Option<terrars::ListField<FsxOntapStorageVirtualMachine_Endpoints_Iscsi>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management: std::option::Option<terrars::ListField<FsxOntapStorageVirtualMachine_Endpoints_Management>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfs: std::option::Option<terrars::ListField<FsxOntapStorageVirtualMachine_Endpoints_Nfs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    smb: std::option::Option<terrars::ListField<FsxOntapStorageVirtualMachine_Endpoints_Smb>>,
}

impl FsxOntapStorageVirtualMachine_Endpoints {
    #[doc = "Set the field `iscsi`.\n"]
    pub fn set_iscsi(
        mut self,
        v: impl std::convert::Into<terrars::ListField<FsxOntapStorageVirtualMachine_Endpoints_Iscsi>>,
    ) -> Self {
        self.iscsi = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `management`.\n"]
    pub fn set_management(
        mut self,
        v: impl std::convert::Into<terrars::ListField<FsxOntapStorageVirtualMachine_Endpoints_Management>>,
    ) -> Self {
        self.management = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nfs`.\n"]
    pub fn set_nfs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<FsxOntapStorageVirtualMachine_Endpoints_Nfs>>,
    ) -> Self {
        self.nfs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `smb`.\n"]
    pub fn set_smb(
        mut self,
        v: impl std::convert::Into<terrars::ListField<FsxOntapStorageVirtualMachine_Endpoints_Smb>>,
    ) -> Self {
        self.smb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxOntapStorageVirtualMachine_Endpoints {
    type O = terrars::BlockAssignable<FsxOntapStorageVirtualMachine_Endpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachine_Endpoints {}

impl BuildFsxOntapStorageVirtualMachine_Endpoints {
    pub fn build(self) -> FsxOntapStorageVirtualMachine_Endpoints {
        FsxOntapStorageVirtualMachine_Endpoints {
            iscsi: core::default::Default::default(),
            management: core::default::Default::default(),
            nfs: core::default::Default::default(),
            smb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOntapStorageVirtualMachine_EndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapStorageVirtualMachine_EndpointsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxOntapStorageVirtualMachine_EndpointsRef {
        FsxOntapStorageVirtualMachine_EndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachine_EndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iscsi` after provisioning.\n"]
    pub fn iscsi(&self) -> terrars::ListRef<FsxOntapStorageVirtualMachine_Endpoints_IscsiRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iscsi", self.base))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> terrars::ListRef<FsxOntapStorageVirtualMachine_Endpoints_ManagementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management", self.base))
    }

    #[doc = "Get a reference to the value of field `nfs` after provisioning.\n"]
    pub fn nfs(&self) -> terrars::ListRef<FsxOntapStorageVirtualMachine_Endpoints_NfsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs", self.base))
    }

    #[doc = "Get a reference to the value of field `smb` after provisioning.\n"]
    pub fn smb(&self) -> terrars::ListRef<FsxOntapStorageVirtualMachine_Endpoints_SmbRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.smb", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration {
    dns_ips: terrars::SetField<terrars::PrimField<std::string::String>>,
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_administrators_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organizational_unit_distinguished_name: std::option::Option<terrars::PrimField<std::string::String>>,
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration {
    #[doc = "Set the field `file_system_administrators_group`.\n"]
    pub fn set_file_system_administrators_group(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.file_system_administrators_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_unit_distinguished_name`.\n"]
    pub fn set_organizational_unit_distinguished_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.organizational_unit_distinguished_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration {
    #[doc = ""]
    pub dns_ips: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration {
    pub fn build(
        self,
    ) -> FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration {
        FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration {
            dns_ips: self.dns_ips,
            domain_name: self.domain_name,
            file_system_administrators_group: core::default::Default::default(),
            organizational_unit_distinguished_name: core::default::Default::default(),
            password: self.password,
            username: self.username,
        }
    }
}

#[allow(dead_code)]
pub struct FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfigurationRef {
        FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_ips` after provisioning.\n"]
    pub fn dns_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dns_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_administrators_group` after provisioning.\n"]
    pub fn file_system_administrators_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_administrators_group", self.base))
    }

    #[doc = "Get a reference to the value of field `organizational_unit_distinguished_name` after provisioning.\n"]
    pub fn organizational_unit_distinguished_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.organizational_unit_distinguished_name", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `password` after provisioning.\n"]
    pub fn password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxOntapStorageVirtualMachine_ActiveDirectoryConfigurationDynamic {
    self_managed_active_directory_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    netbios_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed_active_directory_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration>,
    >,
    dynamic: FsxOntapStorageVirtualMachine_ActiveDirectoryConfigurationDynamic,
}

impl FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration {
    #[doc = "Set the field `netbios_name`.\n"]
    pub fn set_netbios_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.netbios_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `self_managed_active_directory_configuration`.\n"]
    pub fn set_self_managed_active_directory_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.self_managed_active_directory_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.self_managed_active_directory_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration {
    type O = terrars::BlockAssignable<FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration {}

impl BuildFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration {
    pub fn build(self) -> FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration {
        FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration {
            netbios_name: core::default::Default::default(),
            self_managed_active_directory_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOntapStorageVirtualMachine_ActiveDirectoryConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapStorageVirtualMachine_ActiveDirectoryConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOntapStorageVirtualMachine_ActiveDirectoryConfigurationRef {
        FsxOntapStorageVirtualMachine_ActiveDirectoryConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachine_ActiveDirectoryConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `netbios_name` after provisioning.\n"]
    pub fn netbios_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.netbios_name", self.base))
    }

    #[doc = "Get a reference to the value of field `self_managed_active_directory_configuration` after provisioning.\n"]
    pub fn self_managed_active_directory_configuration(
        &self,
    ) -> terrars
    ::ListRef<FsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.self_managed_active_directory_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachine_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxOntapStorageVirtualMachine_Timeouts {
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

impl terrars::ToListMappable for FsxOntapStorageVirtualMachine_Timeouts {
    type O = terrars::BlockAssignable<FsxOntapStorageVirtualMachine_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachine_Timeouts {}

impl BuildFsxOntapStorageVirtualMachine_Timeouts {
    pub fn build(self) -> FsxOntapStorageVirtualMachine_Timeouts {
        FsxOntapStorageVirtualMachine_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxOntapStorageVirtualMachine_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapStorageVirtualMachine_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxOntapStorageVirtualMachine_TimeoutsRef {
        FsxOntapStorageVirtualMachine_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachine_TimeoutsRef {
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
