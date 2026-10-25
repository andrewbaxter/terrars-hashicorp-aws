#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GuarddutyOrganizationConfigurationData_ {
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
    auto_enable_organization_members: terrars::PrimField<std::string::String>,
    detector_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datasources: std::option::Option<std::vec::Vec<GuarddutyOrganizationConfiguration_Datasources>>,
    dynamic: GuarddutyOrganizationConfigurationDynamic,
}

struct GuarddutyOrganizationConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GuarddutyOrganizationConfigurationData_>,
}

#[derive(Clone)]
pub struct GuarddutyOrganizationConfiguration(std::rc::Rc<GuarddutyOrganizationConfiguration_>);

impl GuarddutyOrganizationConfiguration {
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

    #[doc = "Set the field `datasources`.\n"]
    pub fn set_datasources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GuarddutyOrganizationConfiguration_Datasources>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().datasources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.datasources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `auto_enable_organization_members` after provisioning.\n"]
    pub fn auto_enable_organization_members(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_enable_organization_members", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `detector_id` after provisioning.\n"]
    pub fn detector_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detector_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `datasources` after provisioning.\n"]
    pub fn datasources(&self) -> terrars::ListRef<GuarddutyOrganizationConfiguration_DatasourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datasources", self.extract_ref()))
    }
}

impl terrars::Referable for GuarddutyOrganizationConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GuarddutyOrganizationConfiguration { }

impl terrars::ToListMappable for GuarddutyOrganizationConfiguration {
    type O = terrars::ListRef<GuarddutyOrganizationConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GuarddutyOrganizationConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_guardduty_organization_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGuarddutyOrganizationConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub auto_enable_organization_members: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub detector_id: terrars::PrimField<std::string::String>,
}

impl BuildGuarddutyOrganizationConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> GuarddutyOrganizationConfiguration {
        let out = GuarddutyOrganizationConfiguration(std::rc::Rc::new(GuarddutyOrganizationConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GuarddutyOrganizationConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                auto_enable_organization_members: self.auto_enable_organization_members,
                detector_id: self.detector_id,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                datasources: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GuarddutyOrganizationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GuarddutyOrganizationConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GuarddutyOrganizationConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_enable_organization_members` after provisioning.\n"]
    pub fn auto_enable_organization_members(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_enable_organization_members", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `detector_id` after provisioning.\n"]
    pub fn detector_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detector_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `datasources` after provisioning.\n"]
    pub fn datasources(&self) -> terrars::ListRef<GuarddutyOrganizationConfiguration_DatasourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datasources", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct GuarddutyOrganizationConfigurationDynamic {
    datasources: std::option::Option<terrars::DynamicBlock<GuarddutyOrganizationConfiguration_Datasources>>,
}

#[derive(Serialize)]
pub struct GuarddutyOrganizationConfiguration_Datasources_Kubernetes_AuditLogs {
    enable: terrars::PrimField<bool>,
}

impl GuarddutyOrganizationConfiguration_Datasources_Kubernetes_AuditLogs { }

impl terrars::ToListMappable for GuarddutyOrganizationConfiguration_Datasources_Kubernetes_AuditLogs {
    type O = terrars::BlockAssignable<GuarddutyOrganizationConfiguration_Datasources_Kubernetes_AuditLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGuarddutyOrganizationConfiguration_Datasources_Kubernetes_AuditLogs {
    #[doc = ""]
    pub enable: terrars::PrimField<bool>,
}

impl BuildGuarddutyOrganizationConfiguration_Datasources_Kubernetes_AuditLogs {
    pub fn build(self) -> GuarddutyOrganizationConfiguration_Datasources_Kubernetes_AuditLogs {
        GuarddutyOrganizationConfiguration_Datasources_Kubernetes_AuditLogs { enable: self.enable }
    }
}

#[allow(dead_code)]
pub struct GuarddutyOrganizationConfiguration_Datasources_Kubernetes_AuditLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GuarddutyOrganizationConfiguration_Datasources_Kubernetes_AuditLogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GuarddutyOrganizationConfiguration_Datasources_Kubernetes_AuditLogsRef {
        GuarddutyOrganizationConfiguration_Datasources_Kubernetes_AuditLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GuarddutyOrganizationConfiguration_Datasources_Kubernetes_AuditLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
}

#[derive(Serialize, Default)]
struct GuarddutyOrganizationConfiguration_Datasources_KubernetesDynamic {
    audit_logs: std
    ::option
    ::Option<terrars::DynamicBlock<GuarddutyOrganizationConfiguration_Datasources_Kubernetes_AuditLogs>>,
}

#[derive(Serialize)]
pub struct GuarddutyOrganizationConfiguration_Datasources_Kubernetes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit_logs: std::option::Option<std::vec::Vec<GuarddutyOrganizationConfiguration_Datasources_Kubernetes_AuditLogs>>,
    dynamic: GuarddutyOrganizationConfiguration_Datasources_KubernetesDynamic,
}

impl GuarddutyOrganizationConfiguration_Datasources_Kubernetes {
    #[doc = "Set the field `audit_logs`.\n"]
    pub fn set_audit_logs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<GuarddutyOrganizationConfiguration_Datasources_Kubernetes_AuditLogs>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.audit_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.audit_logs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GuarddutyOrganizationConfiguration_Datasources_Kubernetes {
    type O = terrars::BlockAssignable<GuarddutyOrganizationConfiguration_Datasources_Kubernetes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGuarddutyOrganizationConfiguration_Datasources_Kubernetes {}

impl BuildGuarddutyOrganizationConfiguration_Datasources_Kubernetes {
    pub fn build(self) -> GuarddutyOrganizationConfiguration_Datasources_Kubernetes {
        GuarddutyOrganizationConfiguration_Datasources_Kubernetes {
            audit_logs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GuarddutyOrganizationConfiguration_Datasources_KubernetesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GuarddutyOrganizationConfiguration_Datasources_KubernetesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GuarddutyOrganizationConfiguration_Datasources_KubernetesRef {
        GuarddutyOrganizationConfiguration_Datasources_KubernetesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GuarddutyOrganizationConfiguration_Datasources_KubernetesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audit_logs` after provisioning.\n"]
    pub fn audit_logs(&self) -> terrars::ListRef<GuarddutyOrganizationConfiguration_Datasources_Kubernetes_AuditLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audit_logs", self.base))
    }
}

#[derive(Serialize)]
pub struct GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings_EbsVolumes {
    auto_enable: terrars::PrimField<bool>,
}

impl GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings_EbsVolumes { }

impl terrars
::ToListMappable for GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings_EbsVolumes {
    type O =
        terrars
        ::BlockAssignable<
            GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings_EbsVolumes,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings_EbsVolumes {
    #[doc = ""]
    pub auto_enable: terrars::PrimField<bool>,
}

impl BuildGuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings_EbsVolumes {
    pub fn build(
        self,
    ) -> GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings_EbsVolumes {
        GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings_EbsVolumes {
            auto_enable: self.auto_enable,
        }
    }
}

#[allow(dead_code)]
pub struct GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings_EbsVolumesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings_EbsVolumesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings_EbsVolumesRef {
        GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings_EbsVolumesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings_EbsVolumesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_enable` after provisioning.\n"]
    pub fn auto_enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_enable", self.base))
    }
}

#[derive(Serialize, Default)]
struct GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindingsDynamic {
    ebs_volumes: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings_EbsVolumes,
        >,
    >,
}

#[derive(Serialize)]
pub struct GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_volumes: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings_EbsVolumes>,
    >,
    dynamic: GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindingsDynamic,
}

impl GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings {
    #[doc = "Set the field `ebs_volumes`.\n"]
    pub fn set_ebs_volumes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings_EbsVolumes,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ebs_volumes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs_volumes = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings {
    type O =
        terrars
        ::BlockAssignable<
            GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings {}

impl BuildGuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings {
    pub fn build(
        self,
    ) -> GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings {
        GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings {
            ebs_volumes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindingsRef {
        GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ebs_volumes` after provisioning.\n"]
    pub fn ebs_volumes(
        &self,
    ) -> terrars
    ::ListRef<
        GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings_EbsVolumesRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs_volumes", self.base))
    }
}

#[derive(Serialize, Default)]
struct GuarddutyOrganizationConfiguration_Datasources_MalwareProtectionDynamic {
    scan_ec2_instance_with_findings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings>,
    >,
}

#[derive(Serialize)]
pub struct GuarddutyOrganizationConfiguration_Datasources_MalwareProtection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_ec2_instance_with_findings: std
    ::option
    ::Option<
        std::vec::Vec<GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings>,
    >,
    dynamic: GuarddutyOrganizationConfiguration_Datasources_MalwareProtectionDynamic,
}

impl GuarddutyOrganizationConfiguration_Datasources_MalwareProtection {
    #[doc = "Set the field `scan_ec2_instance_with_findings`.\n"]
    pub fn set_scan_ec2_instance_with_findings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scan_ec2_instance_with_findings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scan_ec2_instance_with_findings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GuarddutyOrganizationConfiguration_Datasources_MalwareProtection {
    type O = terrars::BlockAssignable<GuarddutyOrganizationConfiguration_Datasources_MalwareProtection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGuarddutyOrganizationConfiguration_Datasources_MalwareProtection {}

impl BuildGuarddutyOrganizationConfiguration_Datasources_MalwareProtection {
    pub fn build(self) -> GuarddutyOrganizationConfiguration_Datasources_MalwareProtection {
        GuarddutyOrganizationConfiguration_Datasources_MalwareProtection {
            scan_ec2_instance_with_findings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GuarddutyOrganizationConfiguration_Datasources_MalwareProtectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GuarddutyOrganizationConfiguration_Datasources_MalwareProtectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GuarddutyOrganizationConfiguration_Datasources_MalwareProtectionRef {
        GuarddutyOrganizationConfiguration_Datasources_MalwareProtectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GuarddutyOrganizationConfiguration_Datasources_MalwareProtectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scan_ec2_instance_with_findings` after provisioning.\n"]
    pub fn scan_ec2_instance_with_findings(
        &self,
    ) -> terrars
    ::ListRef<GuarddutyOrganizationConfiguration_Datasources_MalwareProtection_ScanEc2InstanceWithFindingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scan_ec2_instance_with_findings", self.base))
    }
}

#[derive(Serialize)]
pub struct GuarddutyOrganizationConfiguration_Datasources_S3Logs {
    auto_enable: terrars::PrimField<bool>,
}

impl GuarddutyOrganizationConfiguration_Datasources_S3Logs { }

impl terrars::ToListMappable for GuarddutyOrganizationConfiguration_Datasources_S3Logs {
    type O = terrars::BlockAssignable<GuarddutyOrganizationConfiguration_Datasources_S3Logs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGuarddutyOrganizationConfiguration_Datasources_S3Logs {
    #[doc = ""]
    pub auto_enable: terrars::PrimField<bool>,
}

impl BuildGuarddutyOrganizationConfiguration_Datasources_S3Logs {
    pub fn build(self) -> GuarddutyOrganizationConfiguration_Datasources_S3Logs {
        GuarddutyOrganizationConfiguration_Datasources_S3Logs { auto_enable: self.auto_enable }
    }
}

#[allow(dead_code)]
pub struct GuarddutyOrganizationConfiguration_Datasources_S3LogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GuarddutyOrganizationConfiguration_Datasources_S3LogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GuarddutyOrganizationConfiguration_Datasources_S3LogsRef {
        GuarddutyOrganizationConfiguration_Datasources_S3LogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GuarddutyOrganizationConfiguration_Datasources_S3LogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_enable` after provisioning.\n"]
    pub fn auto_enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_enable", self.base))
    }
}

#[derive(Serialize, Default)]
struct GuarddutyOrganizationConfiguration_DatasourcesDynamic {
    kubernetes: std::option::Option<terrars::DynamicBlock<GuarddutyOrganizationConfiguration_Datasources_Kubernetes>>,
    malware_protection: std
    ::option
    ::Option<terrars::DynamicBlock<GuarddutyOrganizationConfiguration_Datasources_MalwareProtection>>,
    s3_logs: std::option::Option<terrars::DynamicBlock<GuarddutyOrganizationConfiguration_Datasources_S3Logs>>,
}

#[derive(Serialize)]
pub struct GuarddutyOrganizationConfiguration_Datasources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubernetes: std::option::Option<std::vec::Vec<GuarddutyOrganizationConfiguration_Datasources_Kubernetes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    malware_protection: std
    ::option
    ::Option<std::vec::Vec<GuarddutyOrganizationConfiguration_Datasources_MalwareProtection>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_logs: std::option::Option<std::vec::Vec<GuarddutyOrganizationConfiguration_Datasources_S3Logs>>,
    dynamic: GuarddutyOrganizationConfiguration_DatasourcesDynamic,
}

impl GuarddutyOrganizationConfiguration_Datasources {
    #[doc = "Set the field `kubernetes`.\n"]
    pub fn set_kubernetes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GuarddutyOrganizationConfiguration_Datasources_Kubernetes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kubernetes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kubernetes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `malware_protection`.\n"]
    pub fn set_malware_protection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<GuarddutyOrganizationConfiguration_Datasources_MalwareProtection>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.malware_protection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.malware_protection = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_logs`.\n"]
    pub fn set_s3_logs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GuarddutyOrganizationConfiguration_Datasources_S3Logs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_logs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GuarddutyOrganizationConfiguration_Datasources {
    type O = terrars::BlockAssignable<GuarddutyOrganizationConfiguration_Datasources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGuarddutyOrganizationConfiguration_Datasources {}

impl BuildGuarddutyOrganizationConfiguration_Datasources {
    pub fn build(self) -> GuarddutyOrganizationConfiguration_Datasources {
        GuarddutyOrganizationConfiguration_Datasources {
            kubernetes: core::default::Default::default(),
            malware_protection: core::default::Default::default(),
            s3_logs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GuarddutyOrganizationConfiguration_DatasourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GuarddutyOrganizationConfiguration_DatasourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GuarddutyOrganizationConfiguration_DatasourcesRef {
        GuarddutyOrganizationConfiguration_DatasourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GuarddutyOrganizationConfiguration_DatasourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kubernetes` after provisioning.\n"]
    pub fn kubernetes(&self) -> terrars::ListRef<GuarddutyOrganizationConfiguration_Datasources_KubernetesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubernetes", self.base))
    }

    #[doc = "Get a reference to the value of field `malware_protection` after provisioning.\n"]
    pub fn malware_protection(
        &self,
    ) -> terrars::ListRef<GuarddutyOrganizationConfiguration_Datasources_MalwareProtectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.malware_protection", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_logs` after provisioning.\n"]
    pub fn s3_logs(&self) -> terrars::ListRef<GuarddutyOrganizationConfiguration_Datasources_S3LogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_logs", self.base))
    }
}
