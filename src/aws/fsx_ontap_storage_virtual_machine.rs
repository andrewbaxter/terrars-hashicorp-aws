use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FsxOntapStorageVirtualMachineData {
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
    ::Option<std::vec::Vec<FsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FsxOntapStorageVirtualMachineTimeoutsEl>,
    dynamic: FsxOntapStorageVirtualMachineDynamic,
}

struct FsxOntapStorageVirtualMachine_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FsxOntapStorageVirtualMachineData>,
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
                    ::Into<terrars::BlockAssignable<FsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<FsxOntapStorageVirtualMachineTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::ListRef<FsxOntapStorageVirtualMachineEndpointsElRef> {
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
    ) -> terrars::ListRef<FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.active_directory_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOntapStorageVirtualMachineTimeoutsElRef {
        FsxOntapStorageVirtualMachineTimeoutsElRef::new(
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
            data: std::cell::RefCell::new(FsxOntapStorageVirtualMachineData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                file_system_id: self.file_system_id,
                id: core::default::Default::default(),
                name: self.name,
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
    pub fn endpoints(&self) -> terrars::ListRef<FsxOntapStorageVirtualMachineEndpointsElRef> {
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
    ) -> terrars::ListRef<FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.active_directory_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxOntapStorageVirtualMachineTimeoutsElRef {
        FsxOntapStorageVirtualMachineTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachineEndpointsElIscsiEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl FsxOntapStorageVirtualMachineEndpointsElIscsiEl {
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

impl terrars::ToListMappable for FsxOntapStorageVirtualMachineEndpointsElIscsiEl {
    type O = terrars::BlockAssignable<FsxOntapStorageVirtualMachineEndpointsElIscsiEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachineEndpointsElIscsiEl {}

impl BuildFsxOntapStorageVirtualMachineEndpointsElIscsiEl {
    pub fn build(self) -> FsxOntapStorageVirtualMachineEndpointsElIscsiEl {
        FsxOntapStorageVirtualMachineEndpointsElIscsiEl {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapStorageVirtualMachineEndpointsElIscsiElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapStorageVirtualMachineEndpointsElIscsiElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOntapStorageVirtualMachineEndpointsElIscsiElRef {
        FsxOntapStorageVirtualMachineEndpointsElIscsiElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachineEndpointsElIscsiElRef {
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
pub struct FsxOntapStorageVirtualMachineEndpointsElManagementEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl FsxOntapStorageVirtualMachineEndpointsElManagementEl {
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

impl terrars::ToListMappable for FsxOntapStorageVirtualMachineEndpointsElManagementEl {
    type O = terrars::BlockAssignable<FsxOntapStorageVirtualMachineEndpointsElManagementEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachineEndpointsElManagementEl {}

impl BuildFsxOntapStorageVirtualMachineEndpointsElManagementEl {
    pub fn build(self) -> FsxOntapStorageVirtualMachineEndpointsElManagementEl {
        FsxOntapStorageVirtualMachineEndpointsElManagementEl {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapStorageVirtualMachineEndpointsElManagementElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapStorageVirtualMachineEndpointsElManagementElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOntapStorageVirtualMachineEndpointsElManagementElRef {
        FsxOntapStorageVirtualMachineEndpointsElManagementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachineEndpointsElManagementElRef {
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
pub struct FsxOntapStorageVirtualMachineEndpointsElNfsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl FsxOntapStorageVirtualMachineEndpointsElNfsEl {
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

impl terrars::ToListMappable for FsxOntapStorageVirtualMachineEndpointsElNfsEl {
    type O = terrars::BlockAssignable<FsxOntapStorageVirtualMachineEndpointsElNfsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachineEndpointsElNfsEl {}

impl BuildFsxOntapStorageVirtualMachineEndpointsElNfsEl {
    pub fn build(self) -> FsxOntapStorageVirtualMachineEndpointsElNfsEl {
        FsxOntapStorageVirtualMachineEndpointsElNfsEl {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapStorageVirtualMachineEndpointsElNfsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapStorageVirtualMachineEndpointsElNfsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOntapStorageVirtualMachineEndpointsElNfsElRef {
        FsxOntapStorageVirtualMachineEndpointsElNfsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachineEndpointsElNfsElRef {
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
pub struct FsxOntapStorageVirtualMachineEndpointsElSmbEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl FsxOntapStorageVirtualMachineEndpointsElSmbEl {
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

impl terrars::ToListMappable for FsxOntapStorageVirtualMachineEndpointsElSmbEl {
    type O = terrars::BlockAssignable<FsxOntapStorageVirtualMachineEndpointsElSmbEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachineEndpointsElSmbEl {}

impl BuildFsxOntapStorageVirtualMachineEndpointsElSmbEl {
    pub fn build(self) -> FsxOntapStorageVirtualMachineEndpointsElSmbEl {
        FsxOntapStorageVirtualMachineEndpointsElSmbEl {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapStorageVirtualMachineEndpointsElSmbElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapStorageVirtualMachineEndpointsElSmbElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOntapStorageVirtualMachineEndpointsElSmbElRef {
        FsxOntapStorageVirtualMachineEndpointsElSmbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachineEndpointsElSmbElRef {
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
pub struct FsxOntapStorageVirtualMachineEndpointsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iscsi: std::option::Option<terrars::ListField<FsxOntapStorageVirtualMachineEndpointsElIscsiEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management: std::option::Option<terrars::ListField<FsxOntapStorageVirtualMachineEndpointsElManagementEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfs: std::option::Option<terrars::ListField<FsxOntapStorageVirtualMachineEndpointsElNfsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    smb: std::option::Option<terrars::ListField<FsxOntapStorageVirtualMachineEndpointsElSmbEl>>,
}

impl FsxOntapStorageVirtualMachineEndpointsEl {
    #[doc = "Set the field `iscsi`.\n"]
    pub fn set_iscsi(
        mut self,
        v: impl std::convert::Into<terrars::ListField<FsxOntapStorageVirtualMachineEndpointsElIscsiEl>>,
    ) -> Self {
        self.iscsi = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `management`.\n"]
    pub fn set_management(
        mut self,
        v: impl std::convert::Into<terrars::ListField<FsxOntapStorageVirtualMachineEndpointsElManagementEl>>,
    ) -> Self {
        self.management = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nfs`.\n"]
    pub fn set_nfs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<FsxOntapStorageVirtualMachineEndpointsElNfsEl>>,
    ) -> Self {
        self.nfs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `smb`.\n"]
    pub fn set_smb(
        mut self,
        v: impl std::convert::Into<terrars::ListField<FsxOntapStorageVirtualMachineEndpointsElSmbEl>>,
    ) -> Self {
        self.smb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxOntapStorageVirtualMachineEndpointsEl {
    type O = terrars::BlockAssignable<FsxOntapStorageVirtualMachineEndpointsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachineEndpointsEl {}

impl BuildFsxOntapStorageVirtualMachineEndpointsEl {
    pub fn build(self) -> FsxOntapStorageVirtualMachineEndpointsEl {
        FsxOntapStorageVirtualMachineEndpointsEl {
            iscsi: core::default::Default::default(),
            management: core::default::Default::default(),
            nfs: core::default::Default::default(),
            smb: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapStorageVirtualMachineEndpointsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapStorageVirtualMachineEndpointsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxOntapStorageVirtualMachineEndpointsElRef {
        FsxOntapStorageVirtualMachineEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachineEndpointsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iscsi` after provisioning.\n"]
    pub fn iscsi(&self) -> terrars::ListRef<FsxOntapStorageVirtualMachineEndpointsElIscsiElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iscsi", self.base))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> terrars::ListRef<FsxOntapStorageVirtualMachineEndpointsElManagementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management", self.base))
    }

    #[doc = "Get a reference to the value of field `nfs` after provisioning.\n"]
    pub fn nfs(&self) -> terrars::ListRef<FsxOntapStorageVirtualMachineEndpointsElNfsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs", self.base))
    }

    #[doc = "Get a reference to the value of field `smb` after provisioning.\n"]
    pub fn smb(&self) -> terrars::ListRef<FsxOntapStorageVirtualMachineEndpointsElSmbElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.smb", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl {
    dns_ips: terrars::SetField<terrars::PrimField<std::string::String>>,
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_administrators_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organizational_unit_distinguished_name: std::option::Option<terrars::PrimField<std::string::String>>,
    password: terrars::PrimField<std::string::String>,
    username: terrars::PrimField<std::string::String>,
}

impl FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl {
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
::ToListMappable for FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<
            FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl {
    #[doc = ""]
    pub dns_ips: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub username: terrars::PrimField<std::string::String>,
}

impl BuildFsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl {
    pub fn build(
        self,
    ) -> FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl {
        FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl {
            dns_ips: self.dns_ips,
            domain_name: self.domain_name,
            file_system_administrators_group: core::default::Default::default(),
            organizational_unit_distinguished_name: core::default::Default::default(),
            password: self.password,
            username: self.username,
        }
    }
}

pub struct FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationElRef {
        FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationElRef {
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
struct FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElDynamic {
    self_managed_active_directory_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    netbios_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed_active_directory_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl>,
    >,
    dynamic: FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElDynamic,
}

impl FsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {
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
                            FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationEl,
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

impl terrars::ToListMappable for FsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {
    type O = terrars::BlockAssignable<FsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {}

impl BuildFsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {
    pub fn build(self) -> FsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {
        FsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl {
            netbios_name: core::default::Default::default(),
            self_managed_active_directory_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef {
        FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElRef {
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
    ::ListRef<
        FsxOntapStorageVirtualMachineActiveDirectoryConfigurationElSelfManagedActiveDirectoryConfigurationElRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.self_managed_active_directory_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct FsxOntapStorageVirtualMachineTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxOntapStorageVirtualMachineTimeoutsEl {
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

impl terrars::ToListMappable for FsxOntapStorageVirtualMachineTimeoutsEl {
    type O = terrars::BlockAssignable<FsxOntapStorageVirtualMachineTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxOntapStorageVirtualMachineTimeoutsEl {}

impl BuildFsxOntapStorageVirtualMachineTimeoutsEl {
    pub fn build(self) -> FsxOntapStorageVirtualMachineTimeoutsEl {
        FsxOntapStorageVirtualMachineTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct FsxOntapStorageVirtualMachineTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxOntapStorageVirtualMachineTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxOntapStorageVirtualMachineTimeoutsElRef {
        FsxOntapStorageVirtualMachineTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxOntapStorageVirtualMachineTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct FsxOntapStorageVirtualMachineDynamic {
    active_directory_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FsxOntapStorageVirtualMachineActiveDirectoryConfigurationEl>>,
}
