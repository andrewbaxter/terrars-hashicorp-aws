#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataFsxOntapStorageVirtualMachineData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataFsxOntapStorageVirtualMachine_Filter>>,
    dynamic: DataFsxOntapStorageVirtualMachineDynamic,
}

struct DataFsxOntapStorageVirtualMachine_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataFsxOntapStorageVirtualMachineData_>,
}

#[derive(Clone)]
pub struct DataFsxOntapStorageVirtualMachine(std::rc::Rc<DataFsxOntapStorageVirtualMachine_>);

impl DataFsxOntapStorageVirtualMachine {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataFsxOntapStorageVirtualMachine_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `active_directory_configuration` after provisioning.\n"]
    pub fn active_directory_configuration(
        &self,
    ) -> terrars::ListRef<DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.active_directory_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::ListRef<DataFsxOntapStorageVirtualMachine_EndpointsRef> {
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

    #[doc = "Get a reference to the value of field `lifecycle_status` after provisioning.\n"]
    pub fn lifecycle_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_transition_reason` after provisioning.\n"]
    pub fn lifecycle_transition_reason(
        &self,
    ) -> terrars::SetRef<DataFsxOntapStorageVirtualMachine_LifecycleTransitionReasonRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.lifecycle_transition_reason", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `subtype` after provisioning.\n"]
    pub fn subtype(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subtype", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }
}

impl terrars::Referable for DataFsxOntapStorageVirtualMachine {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataFsxOntapStorageVirtualMachine { }

impl terrars::ToListMappable for DataFsxOntapStorageVirtualMachine {
    type O = terrars::ListRef<DataFsxOntapStorageVirtualMachineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataFsxOntapStorageVirtualMachine_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_fsx_ontap_storage_virtual_machine".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachine {
    pub tf_id: std::string::String,
}

impl BuildDataFsxOntapStorageVirtualMachine {
    pub fn build(self, stack: &mut terrars::Stack) -> DataFsxOntapStorageVirtualMachine {
        let out = DataFsxOntapStorageVirtualMachine(std::rc::Rc::new(DataFsxOntapStorageVirtualMachine_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataFsxOntapStorageVirtualMachineData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataFsxOntapStorageVirtualMachineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFsxOntapStorageVirtualMachineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataFsxOntapStorageVirtualMachineRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `active_directory_configuration` after provisioning.\n"]
    pub fn active_directory_configuration(
        &self,
    ) -> terrars::ListRef<DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.active_directory_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoints` after provisioning.\n"]
    pub fn endpoints(&self) -> terrars::ListRef<DataFsxOntapStorageVirtualMachine_EndpointsRef> {
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

    #[doc = "Get a reference to the value of field `lifecycle_status` after provisioning.\n"]
    pub fn lifecycle_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lifecycle_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_transition_reason` after provisioning.\n"]
    pub fn lifecycle_transition_reason(
        &self,
    ) -> terrars::SetRef<DataFsxOntapStorageVirtualMachine_LifecycleTransitionReasonRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.lifecycle_transition_reason", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `subtype` after provisioning.\n"]
    pub fn subtype(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subtype", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uuid` after provisioning.\n"]
    pub fn uuid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uuid", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataFsxOntapStorageVirtualMachineDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataFsxOntapStorageVirtualMachine_Filter>>,
}

#[derive(Serialize)]
pub struct DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_ips: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_administrators_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organizational_unit_distinguished_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration {
    #[doc = "Set the field `dns_ips`.\n"]
    pub fn set_dns_ips(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dns_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain_name = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration {}

impl BuildDataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration {
    pub fn build(
        self,
    ) -> DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration {
        DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration {
            dns_ips: core::default::Default::default(),
            domain_name: core::default::Default::default(),
            file_system_administrators_group: core::default::Default::default(),
            organizational_unit_distinguished_name: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfigurationRef {
        DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfigurationRef {
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

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    netbios_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    self_managed_active_directory_configuration: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration,
        >,
    >,
}

impl DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration {
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
                        ::ListField<
                            DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfiguration,
                        >,
                    >,
    ) -> Self {
        self.self_managed_active_directory_configuration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration {
    type O = terrars::BlockAssignable<DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration {}

impl BuildDataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration {
    pub fn build(self) -> DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration {
        DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration {
            netbios_name: core::default::Default::default(),
            self_managed_active_directory_configuration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfigurationRef {
        DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfigurationRef {
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
    ::ListRef<
        DataFsxOntapStorageVirtualMachine_ActiveDirectoryConfiguration_SelfManagedActiveDirectoryConfigurationRef,
    > {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.self_managed_active_directory_configuration", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataFsxOntapStorageVirtualMachine_Endpoints_Iscsi {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataFsxOntapStorageVirtualMachine_Endpoints_Iscsi {
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

impl terrars::ToListMappable for DataFsxOntapStorageVirtualMachine_Endpoints_Iscsi {
    type O = terrars::BlockAssignable<DataFsxOntapStorageVirtualMachine_Endpoints_Iscsi>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachine_Endpoints_Iscsi {}

impl BuildDataFsxOntapStorageVirtualMachine_Endpoints_Iscsi {
    pub fn build(self) -> DataFsxOntapStorageVirtualMachine_Endpoints_Iscsi {
        DataFsxOntapStorageVirtualMachine_Endpoints_Iscsi {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFsxOntapStorageVirtualMachine_Endpoints_IscsiRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFsxOntapStorageVirtualMachine_Endpoints_IscsiRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFsxOntapStorageVirtualMachine_Endpoints_IscsiRef {
        DataFsxOntapStorageVirtualMachine_Endpoints_IscsiRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxOntapStorageVirtualMachine_Endpoints_IscsiRef {
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
pub struct DataFsxOntapStorageVirtualMachine_Endpoints_Management {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataFsxOntapStorageVirtualMachine_Endpoints_Management {
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

impl terrars::ToListMappable for DataFsxOntapStorageVirtualMachine_Endpoints_Management {
    type O = terrars::BlockAssignable<DataFsxOntapStorageVirtualMachine_Endpoints_Management>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachine_Endpoints_Management {}

impl BuildDataFsxOntapStorageVirtualMachine_Endpoints_Management {
    pub fn build(self) -> DataFsxOntapStorageVirtualMachine_Endpoints_Management {
        DataFsxOntapStorageVirtualMachine_Endpoints_Management {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFsxOntapStorageVirtualMachine_Endpoints_ManagementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFsxOntapStorageVirtualMachine_Endpoints_ManagementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFsxOntapStorageVirtualMachine_Endpoints_ManagementRef {
        DataFsxOntapStorageVirtualMachine_Endpoints_ManagementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxOntapStorageVirtualMachine_Endpoints_ManagementRef {
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
pub struct DataFsxOntapStorageVirtualMachine_Endpoints_Nfs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataFsxOntapStorageVirtualMachine_Endpoints_Nfs {
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

impl terrars::ToListMappable for DataFsxOntapStorageVirtualMachine_Endpoints_Nfs {
    type O = terrars::BlockAssignable<DataFsxOntapStorageVirtualMachine_Endpoints_Nfs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachine_Endpoints_Nfs {}

impl BuildDataFsxOntapStorageVirtualMachine_Endpoints_Nfs {
    pub fn build(self) -> DataFsxOntapStorageVirtualMachine_Endpoints_Nfs {
        DataFsxOntapStorageVirtualMachine_Endpoints_Nfs {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFsxOntapStorageVirtualMachine_Endpoints_NfsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFsxOntapStorageVirtualMachine_Endpoints_NfsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFsxOntapStorageVirtualMachine_Endpoints_NfsRef {
        DataFsxOntapStorageVirtualMachine_Endpoints_NfsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxOntapStorageVirtualMachine_Endpoints_NfsRef {
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
pub struct DataFsxOntapStorageVirtualMachine_Endpoints_Smb {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataFsxOntapStorageVirtualMachine_Endpoints_Smb {
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

impl terrars::ToListMappable for DataFsxOntapStorageVirtualMachine_Endpoints_Smb {
    type O = terrars::BlockAssignable<DataFsxOntapStorageVirtualMachine_Endpoints_Smb>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachine_Endpoints_Smb {}

impl BuildDataFsxOntapStorageVirtualMachine_Endpoints_Smb {
    pub fn build(self) -> DataFsxOntapStorageVirtualMachine_Endpoints_Smb {
        DataFsxOntapStorageVirtualMachine_Endpoints_Smb {
            dns_name: core::default::Default::default(),
            ip_addresses: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFsxOntapStorageVirtualMachine_Endpoints_SmbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFsxOntapStorageVirtualMachine_Endpoints_SmbRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFsxOntapStorageVirtualMachine_Endpoints_SmbRef {
        DataFsxOntapStorageVirtualMachine_Endpoints_SmbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxOntapStorageVirtualMachine_Endpoints_SmbRef {
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
pub struct DataFsxOntapStorageVirtualMachine_Endpoints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iscsi: std::option::Option<terrars::ListField<DataFsxOntapStorageVirtualMachine_Endpoints_Iscsi>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    management: std::option::Option<terrars::ListField<DataFsxOntapStorageVirtualMachine_Endpoints_Management>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfs: std::option::Option<terrars::ListField<DataFsxOntapStorageVirtualMachine_Endpoints_Nfs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    smb: std::option::Option<terrars::ListField<DataFsxOntapStorageVirtualMachine_Endpoints_Smb>>,
}

impl DataFsxOntapStorageVirtualMachine_Endpoints {
    #[doc = "Set the field `iscsi`.\n"]
    pub fn set_iscsi(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataFsxOntapStorageVirtualMachine_Endpoints_Iscsi>>,
    ) -> Self {
        self.iscsi = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `management`.\n"]
    pub fn set_management(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataFsxOntapStorageVirtualMachine_Endpoints_Management>>,
    ) -> Self {
        self.management = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nfs`.\n"]
    pub fn set_nfs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataFsxOntapStorageVirtualMachine_Endpoints_Nfs>>,
    ) -> Self {
        self.nfs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `smb`.\n"]
    pub fn set_smb(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataFsxOntapStorageVirtualMachine_Endpoints_Smb>>,
    ) -> Self {
        self.smb = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFsxOntapStorageVirtualMachine_Endpoints {
    type O = terrars::BlockAssignable<DataFsxOntapStorageVirtualMachine_Endpoints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachine_Endpoints {}

impl BuildDataFsxOntapStorageVirtualMachine_Endpoints {
    pub fn build(self) -> DataFsxOntapStorageVirtualMachine_Endpoints {
        DataFsxOntapStorageVirtualMachine_Endpoints {
            iscsi: core::default::Default::default(),
            management: core::default::Default::default(),
            nfs: core::default::Default::default(),
            smb: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataFsxOntapStorageVirtualMachine_EndpointsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFsxOntapStorageVirtualMachine_EndpointsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFsxOntapStorageVirtualMachine_EndpointsRef {
        DataFsxOntapStorageVirtualMachine_EndpointsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxOntapStorageVirtualMachine_EndpointsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iscsi` after provisioning.\n"]
    pub fn iscsi(&self) -> terrars::ListRef<DataFsxOntapStorageVirtualMachine_Endpoints_IscsiRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iscsi", self.base))
    }

    #[doc = "Get a reference to the value of field `management` after provisioning.\n"]
    pub fn management(&self) -> terrars::ListRef<DataFsxOntapStorageVirtualMachine_Endpoints_ManagementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.management", self.base))
    }

    #[doc = "Get a reference to the value of field `nfs` after provisioning.\n"]
    pub fn nfs(&self) -> terrars::ListRef<DataFsxOntapStorageVirtualMachine_Endpoints_NfsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs", self.base))
    }

    #[doc = "Get a reference to the value of field `smb` after provisioning.\n"]
    pub fn smb(&self) -> terrars::ListRef<DataFsxOntapStorageVirtualMachine_Endpoints_SmbRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.smb", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFsxOntapStorageVirtualMachine_LifecycleTransitionReason {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataFsxOntapStorageVirtualMachine_LifecycleTransitionReason {
    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataFsxOntapStorageVirtualMachine_LifecycleTransitionReason {
    type O = terrars::BlockAssignable<DataFsxOntapStorageVirtualMachine_LifecycleTransitionReason>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachine_LifecycleTransitionReason {}

impl BuildDataFsxOntapStorageVirtualMachine_LifecycleTransitionReason {
    pub fn build(self) -> DataFsxOntapStorageVirtualMachine_LifecycleTransitionReason {
        DataFsxOntapStorageVirtualMachine_LifecycleTransitionReason { message: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataFsxOntapStorageVirtualMachine_LifecycleTransitionReasonRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFsxOntapStorageVirtualMachine_LifecycleTransitionReasonRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataFsxOntapStorageVirtualMachine_LifecycleTransitionReasonRef {
        DataFsxOntapStorageVirtualMachine_LifecycleTransitionReasonRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxOntapStorageVirtualMachine_LifecycleTransitionReasonRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct DataFsxOntapStorageVirtualMachine_Filter {
    name: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataFsxOntapStorageVirtualMachine_Filter { }

impl terrars::ToListMappable for DataFsxOntapStorageVirtualMachine_Filter {
    type O = terrars::BlockAssignable<DataFsxOntapStorageVirtualMachine_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataFsxOntapStorageVirtualMachine_Filter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataFsxOntapStorageVirtualMachine_Filter {
    pub fn build(self) -> DataFsxOntapStorageVirtualMachine_Filter {
        DataFsxOntapStorageVirtualMachine_Filter {
            name: self.name,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct DataFsxOntapStorageVirtualMachine_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataFsxOntapStorageVirtualMachine_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataFsxOntapStorageVirtualMachine_FilterRef {
        DataFsxOntapStorageVirtualMachine_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataFsxOntapStorageVirtualMachine_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
