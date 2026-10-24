use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GuarddutyOrganizationConfigurationData {
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
    auto_enable: terrars::PrimField<bool>,
    detector_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datasources: std::option::Option<std::vec::Vec<GuarddutyOrganizationConfigurationDatasourcesEl>>,
    dynamic: GuarddutyOrganizationConfigurationDynamic,
}

struct GuarddutyOrganizationConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GuarddutyOrganizationConfigurationData>,
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

    #[doc = "Set the field `datasources`.\n"]
    pub fn set_datasources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesEl>>,
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

    #[doc = "Get a reference to the value of field `auto_enable` after provisioning.\n"]
    pub fn auto_enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_enable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detector_id` after provisioning.\n"]
    pub fn detector_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detector_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `datasources` after provisioning.\n"]
    pub fn datasources(&self) -> terrars::ListRef<GuarddutyOrganizationConfigurationDatasourcesElRef> {
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
    pub auto_enable: terrars::PrimField<bool>,
    #[doc = ""]
    pub detector_id: terrars::PrimField<std::string::String>,
}

impl BuildGuarddutyOrganizationConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> GuarddutyOrganizationConfiguration {
        let out = GuarddutyOrganizationConfiguration(std::rc::Rc::new(GuarddutyOrganizationConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GuarddutyOrganizationConfigurationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                auto_enable: self.auto_enable,
                detector_id: self.detector_id,
                id: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `auto_enable` after provisioning.\n"]
    pub fn auto_enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_enable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detector_id` after provisioning.\n"]
    pub fn detector_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detector_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `datasources` after provisioning.\n"]
    pub fn datasources(&self) -> terrars::ListRef<GuarddutyOrganizationConfigurationDatasourcesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.datasources", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl {
    enable: terrars::PrimField<bool>,
}

impl GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl { }

impl terrars::ToListMappable for GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl {
    type O = terrars::BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl {
    #[doc = ""]
    pub enable: terrars::PrimField<bool>,
}

impl BuildGuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl {
    pub fn build(self) -> GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl {
        GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl { enable: self.enable }
    }
}

pub struct GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsElRef {
        GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }
}

#[derive(Serialize, Default)]
struct GuarddutyOrganizationConfigurationDatasourcesElKubernetesElDynamic {
    audit_logs: std
    ::option
    ::Option<terrars::DynamicBlock<GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl>>,
}

#[derive(Serialize)]
pub struct GuarddutyOrganizationConfigurationDatasourcesElKubernetesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit_logs: std
    ::option
    ::Option<std::vec::Vec<GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl>>,
    dynamic: GuarddutyOrganizationConfigurationDatasourcesElKubernetesElDynamic,
}

impl GuarddutyOrganizationConfigurationDatasourcesElKubernetesEl {
    #[doc = "Set the field `audit_logs`.\n"]
    pub fn set_audit_logs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsEl>,
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

impl terrars::ToListMappable for GuarddutyOrganizationConfigurationDatasourcesElKubernetesEl {
    type O = terrars::BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesElKubernetesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGuarddutyOrganizationConfigurationDatasourcesElKubernetesEl {}

impl BuildGuarddutyOrganizationConfigurationDatasourcesElKubernetesEl {
    pub fn build(self) -> GuarddutyOrganizationConfigurationDatasourcesElKubernetesEl {
        GuarddutyOrganizationConfigurationDatasourcesElKubernetesEl {
            audit_logs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GuarddutyOrganizationConfigurationDatasourcesElKubernetesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GuarddutyOrganizationConfigurationDatasourcesElKubernetesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GuarddutyOrganizationConfigurationDatasourcesElKubernetesElRef {
        GuarddutyOrganizationConfigurationDatasourcesElKubernetesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GuarddutyOrganizationConfigurationDatasourcesElKubernetesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `audit_logs` after provisioning.\n"]
    pub fn audit_logs(
        &self,
    ) -> terrars::ListRef<GuarddutyOrganizationConfigurationDatasourcesElKubernetesElAuditLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.audit_logs", self.base))
    }
}

#[derive(Serialize)]
pub struct GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl {
    auto_enable: terrars::PrimField<bool>,
}

impl GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl { }

impl terrars
::ToListMappable for GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl {
    type O =
        terrars
        ::BlockAssignable<
            GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl {
    #[doc = ""]
    pub auto_enable: terrars::PrimField<bool>,
}

impl BuildGuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl {
    pub fn build(
        self,
    ) -> GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl {
        GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl {
            auto_enable: self.auto_enable,
        }
    }
}

pub struct GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesElRef {
        GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_enable` after provisioning.\n"]
    pub fn auto_enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_enable", self.base))
    }
}

#[derive(Serialize, Default)]
struct GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElDynamic {
    ebs_volumes: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_volumes: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl,
        >,
    >,
    dynamic: GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElDynamic,
}

impl GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl {
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
                            GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesEl,
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
::ToListMappable for GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl {
    type O =
        terrars
        ::BlockAssignable<
            GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl {}

impl BuildGuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl {
    pub fn build(
        self,
    ) -> GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl {
        GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl {
            ebs_volumes: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElRef {
        GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ebs_volumes` after provisioning.\n"]
    pub fn ebs_volumes(
        &self,
    ) -> terrars
    ::ListRef<
        GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElEbsVolumesElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs_volumes", self.base))
    }
}

#[derive(Serialize, Default)]
struct GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElDynamic {
    scan_ec2_instance_with_findings: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_ec2_instance_with_findings: std
    ::option
    ::Option<
        std::vec::Vec<GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl>,
    >,
    dynamic: GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElDynamic,
}

impl GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl {
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
                            GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsEl,
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

impl terrars::ToListMappable for GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl {
    type O = terrars::BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl {}

impl BuildGuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl {
    pub fn build(self) -> GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl {
        GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl {
            scan_ec2_instance_with_findings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElRef {
        GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scan_ec2_instance_with_findings` after provisioning.\n"]
    pub fn scan_ec2_instance_with_findings(
        &self,
    ) -> terrars
    ::ListRef<GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElScanEc2InstanceWithFindingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scan_ec2_instance_with_findings", self.base))
    }
}

#[derive(Serialize)]
pub struct GuarddutyOrganizationConfigurationDatasourcesElS3LogsEl {
    auto_enable: terrars::PrimField<bool>,
}

impl GuarddutyOrganizationConfigurationDatasourcesElS3LogsEl { }

impl terrars::ToListMappable for GuarddutyOrganizationConfigurationDatasourcesElS3LogsEl {
    type O = terrars::BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesElS3LogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGuarddutyOrganizationConfigurationDatasourcesElS3LogsEl {
    #[doc = ""]
    pub auto_enable: terrars::PrimField<bool>,
}

impl BuildGuarddutyOrganizationConfigurationDatasourcesElS3LogsEl {
    pub fn build(self) -> GuarddutyOrganizationConfigurationDatasourcesElS3LogsEl {
        GuarddutyOrganizationConfigurationDatasourcesElS3LogsEl { auto_enable: self.auto_enable }
    }
}

pub struct GuarddutyOrganizationConfigurationDatasourcesElS3LogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GuarddutyOrganizationConfigurationDatasourcesElS3LogsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GuarddutyOrganizationConfigurationDatasourcesElS3LogsElRef {
        GuarddutyOrganizationConfigurationDatasourcesElS3LogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GuarddutyOrganizationConfigurationDatasourcesElS3LogsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_enable` after provisioning.\n"]
    pub fn auto_enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_enable", self.base))
    }
}

#[derive(Serialize, Default)]
struct GuarddutyOrganizationConfigurationDatasourcesElDynamic {
    kubernetes: std
    ::option
    ::Option<terrars::DynamicBlock<GuarddutyOrganizationConfigurationDatasourcesElKubernetesEl>>,
    malware_protection: std
    ::option
    ::Option<terrars::DynamicBlock<GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl>>,
    s3_logs: std::option::Option<terrars::DynamicBlock<GuarddutyOrganizationConfigurationDatasourcesElS3LogsEl>>,
}

#[derive(Serialize)]
pub struct GuarddutyOrganizationConfigurationDatasourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kubernetes: std::option::Option<std::vec::Vec<GuarddutyOrganizationConfigurationDatasourcesElKubernetesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    malware_protection: std
    ::option
    ::Option<std::vec::Vec<GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_logs: std::option::Option<std::vec::Vec<GuarddutyOrganizationConfigurationDatasourcesElS3LogsEl>>,
    dynamic: GuarddutyOrganizationConfigurationDatasourcesElDynamic,
}

impl GuarddutyOrganizationConfigurationDatasourcesEl {
    #[doc = "Set the field `kubernetes`.\n"]
    pub fn set_kubernetes(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesElKubernetesEl>>,
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
                        terrars::BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionEl>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesElS3LogsEl>>,
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

impl terrars::ToListMappable for GuarddutyOrganizationConfigurationDatasourcesEl {
    type O = terrars::BlockAssignable<GuarddutyOrganizationConfigurationDatasourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGuarddutyOrganizationConfigurationDatasourcesEl {}

impl BuildGuarddutyOrganizationConfigurationDatasourcesEl {
    pub fn build(self) -> GuarddutyOrganizationConfigurationDatasourcesEl {
        GuarddutyOrganizationConfigurationDatasourcesEl {
            kubernetes: core::default::Default::default(),
            malware_protection: core::default::Default::default(),
            s3_logs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GuarddutyOrganizationConfigurationDatasourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GuarddutyOrganizationConfigurationDatasourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GuarddutyOrganizationConfigurationDatasourcesElRef {
        GuarddutyOrganizationConfigurationDatasourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GuarddutyOrganizationConfigurationDatasourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kubernetes` after provisioning.\n"]
    pub fn kubernetes(&self) -> terrars::ListRef<GuarddutyOrganizationConfigurationDatasourcesElKubernetesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kubernetes", self.base))
    }

    #[doc = "Get a reference to the value of field `malware_protection` after provisioning.\n"]
    pub fn malware_protection(
        &self,
    ) -> terrars::ListRef<GuarddutyOrganizationConfigurationDatasourcesElMalwareProtectionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.malware_protection", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_logs` after provisioning.\n"]
    pub fn s3_logs(&self) -> terrars::ListRef<GuarddutyOrganizationConfigurationDatasourcesElS3LogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_logs", self.base))
    }
}

#[derive(Serialize, Default)]
struct GuarddutyOrganizationConfigurationDynamic {
    datasources: std::option::Option<terrars::DynamicBlock<GuarddutyOrganizationConfigurationDatasourcesEl>>,
}
