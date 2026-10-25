#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBackupPlanData_ {
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
    plan_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataBackupPlan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBackupPlanData_>,
}

#[derive(Clone)]
pub struct DataBackupPlan(std::rc::Rc<DataBackupPlan_>);

impl DataBackupPlan {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plan_id` after provisioning.\n"]
    pub fn plan_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plan_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::SetRef<DataBackupPlan_RuleRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scan_setting` after provisioning.\n"]
    pub fn scan_setting(&self) -> terrars::SetRef<DataBackupPlan_ScanSettingRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.scan_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl terrars::Referable for DataBackupPlan {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBackupPlan { }

impl terrars::ToListMappable for DataBackupPlan {
    type O = terrars::ListRef<DataBackupPlanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBackupPlan_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_backup_plan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBackupPlan {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub plan_id: terrars::PrimField<std::string::String>,
}

impl BuildDataBackupPlan {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBackupPlan {
        let out = DataBackupPlan(std::rc::Rc::new(DataBackupPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBackupPlanData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                plan_id: self.plan_id,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBackupPlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupPlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBackupPlanRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plan_id` after provisioning.\n"]
    pub fn plan_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plan_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::SetRef<DataBackupPlan_RuleRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scan_setting` after provisioning.\n"]
    pub fn scan_setting(&self) -> terrars::SetRef<DataBackupPlan_ScanSettingRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.scan_setting", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBackupPlan_Rule_Lifecycle {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cold_storage_after: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_after: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    opt_in_to_archive_for_supported_resources: std::option::Option<terrars::PrimField<bool>>,
}

impl DataBackupPlan_Rule_Lifecycle {
    #[doc = "Set the field `cold_storage_after`.\n"]
    pub fn set_cold_storage_after(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cold_storage_after = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete_after`.\n"]
    pub fn set_delete_after(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.delete_after = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `opt_in_to_archive_for_supported_resources`.\n"]
    pub fn set_opt_in_to_archive_for_supported_resources(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.opt_in_to_archive_for_supported_resources = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupPlan_Rule_Lifecycle {
    type O = terrars::BlockAssignable<DataBackupPlan_Rule_Lifecycle>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupPlan_Rule_Lifecycle {}

impl BuildDataBackupPlan_Rule_Lifecycle {
    pub fn build(self) -> DataBackupPlan_Rule_Lifecycle {
        DataBackupPlan_Rule_Lifecycle {
            cold_storage_after: core::default::Default::default(),
            delete_after: core::default::Default::default(),
            opt_in_to_archive_for_supported_resources: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupPlan_Rule_LifecycleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupPlan_Rule_LifecycleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBackupPlan_Rule_LifecycleRef {
        DataBackupPlan_Rule_LifecycleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupPlan_Rule_LifecycleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cold_storage_after` after provisioning.\n"]
    pub fn cold_storage_after(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cold_storage_after", self.base))
    }

    #[doc = "Get a reference to the value of field `delete_after` after provisioning.\n"]
    pub fn delete_after(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_after", self.base))
    }

    #[doc = "Get a reference to the value of field `opt_in_to_archive_for_supported_resources` after provisioning.\n"]
    pub fn opt_in_to_archive_for_supported_resources(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.opt_in_to_archive_for_supported_resources", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct DataBackupPlan_Rule_CopyAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_vault_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle: std::option::Option<terrars::ListField<DataBackupPlan_Rule_Lifecycle>>,
}

impl DataBackupPlan_Rule_CopyAction {
    #[doc = "Set the field `destination_vault_arn`.\n"]
    pub fn set_destination_vault_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.destination_vault_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle`.\n"]
    pub fn set_lifecycle(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBackupPlan_Rule_Lifecycle>>,
    ) -> Self {
        self.lifecycle = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupPlan_Rule_CopyAction {
    type O = terrars::BlockAssignable<DataBackupPlan_Rule_CopyAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupPlan_Rule_CopyAction {}

impl BuildDataBackupPlan_Rule_CopyAction {
    pub fn build(self) -> DataBackupPlan_Rule_CopyAction {
        DataBackupPlan_Rule_CopyAction {
            destination_vault_arn: core::default::Default::default(),
            lifecycle: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupPlan_Rule_CopyActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupPlan_Rule_CopyActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBackupPlan_Rule_CopyActionRef {
        DataBackupPlan_Rule_CopyActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupPlan_Rule_CopyActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_vault_arn` after provisioning.\n"]
    pub fn destination_vault_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_vault_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle` after provisioning.\n"]
    pub fn lifecycle(&self) -> terrars::ListRef<DataBackupPlan_Rule_LifecycleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBackupPlan_Rule_ScanAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    malware_scanner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupPlan_Rule_ScanAction {
    #[doc = "Set the field `malware_scanner`.\n"]
    pub fn set_malware_scanner(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.malware_scanner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scan_mode`.\n"]
    pub fn set_scan_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scan_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupPlan_Rule_ScanAction {
    type O = terrars::BlockAssignable<DataBackupPlan_Rule_ScanAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupPlan_Rule_ScanAction {}

impl BuildDataBackupPlan_Rule_ScanAction {
    pub fn build(self) -> DataBackupPlan_Rule_ScanAction {
        DataBackupPlan_Rule_ScanAction {
            malware_scanner: core::default::Default::default(),
            scan_mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupPlan_Rule_ScanActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupPlan_Rule_ScanActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBackupPlan_Rule_ScanActionRef {
        DataBackupPlan_Rule_ScanActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupPlan_Rule_ScanActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `malware_scanner` after provisioning.\n"]
    pub fn malware_scanner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.malware_scanner", self.base))
    }

    #[doc = "Get a reference to the value of field `scan_mode` after provisioning.\n"]
    pub fn scan_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBackupPlan_Rule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    completion_window: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_action: std::option::Option<terrars::SetField<DataBackupPlan_Rule_CopyAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_continuous_backup: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle: std::option::Option<terrars::ListField<DataBackupPlan_Rule_Lifecycle>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recovery_point_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_action: std::option::Option<terrars::SetField<DataBackupPlan_Rule_ScanAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule_expression_timezone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_window: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_logically_air_gapped_backup_vault_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_vault_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupPlan_Rule {
    #[doc = "Set the field `completion_window`.\n"]
    pub fn set_completion_window(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.completion_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `copy_action`.\n"]
    pub fn set_copy_action(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataBackupPlan_Rule_CopyAction>>,
    ) -> Self {
        self.copy_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_continuous_backup`.\n"]
    pub fn set_enable_continuous_backup(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_continuous_backup = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle`.\n"]
    pub fn set_lifecycle(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBackupPlan_Rule_Lifecycle>>,
    ) -> Self {
        self.lifecycle = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recovery_point_tags`.\n"]
    pub fn set_recovery_point_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.recovery_point_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_name`.\n"]
    pub fn set_rule_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scan_action`.\n"]
    pub fn set_scan_action(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataBackupPlan_Rule_ScanAction>>,
    ) -> Self {
        self.scan_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `schedule_expression_timezone`.\n"]
    pub fn set_schedule_expression_timezone(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.schedule_expression_timezone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_window`.\n"]
    pub fn set_start_window(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.start_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_logically_air_gapped_backup_vault_arn`.\n"]
    pub fn set_target_logically_air_gapped_backup_vault_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.target_logically_air_gapped_backup_vault_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_vault_name`.\n"]
    pub fn set_target_vault_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_vault_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupPlan_Rule {
    type O = terrars::BlockAssignable<DataBackupPlan_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupPlan_Rule {}

impl BuildDataBackupPlan_Rule {
    pub fn build(self) -> DataBackupPlan_Rule {
        DataBackupPlan_Rule {
            completion_window: core::default::Default::default(),
            copy_action: core::default::Default::default(),
            enable_continuous_backup: core::default::Default::default(),
            lifecycle: core::default::Default::default(),
            recovery_point_tags: core::default::Default::default(),
            rule_name: core::default::Default::default(),
            scan_action: core::default::Default::default(),
            schedule: core::default::Default::default(),
            schedule_expression_timezone: core::default::Default::default(),
            start_window: core::default::Default::default(),
            target_logically_air_gapped_backup_vault_arn: core::default::Default::default(),
            target_vault_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupPlan_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupPlan_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBackupPlan_RuleRef {
        DataBackupPlan_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupPlan_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `completion_window` after provisioning.\n"]
    pub fn completion_window(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completion_window", self.base))
    }

    #[doc = "Get a reference to the value of field `copy_action` after provisioning.\n"]
    pub fn copy_action(&self) -> terrars::SetRef<DataBackupPlan_Rule_CopyActionRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.copy_action", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_continuous_backup` after provisioning.\n"]
    pub fn enable_continuous_backup(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_continuous_backup", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle` after provisioning.\n"]
    pub fn lifecycle(&self) -> terrars::ListRef<DataBackupPlan_Rule_LifecycleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle", self.base))
    }

    #[doc = "Get a reference to the value of field `recovery_point_tags` after provisioning.\n"]
    pub fn recovery_point_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.recovery_point_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.base))
    }

    #[doc = "Get a reference to the value of field `scan_action` after provisioning.\n"]
    pub fn scan_action(&self) -> terrars::SetRef<DataBackupPlan_Rule_ScanActionRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.scan_action", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule_expression_timezone` after provisioning.\n"]
    pub fn schedule_expression_timezone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression_timezone", self.base))
    }

    #[doc = "Get a reference to the value of field `start_window` after provisioning.\n"]
    pub fn start_window(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_window", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_logically_air_gapped_backup_vault_arn` after provisioning.\n"]
    pub fn target_logically_air_gapped_backup_vault_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.target_logically_air_gapped_backup_vault_arn", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `target_vault_name` after provisioning.\n"]
    pub fn target_vault_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_vault_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBackupPlan_ScanSetting {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    malware_scanner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scanner_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBackupPlan_ScanSetting {
    #[doc = "Set the field `malware_scanner`.\n"]
    pub fn set_malware_scanner(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.malware_scanner = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_types`.\n"]
    pub fn set_resource_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scanner_role_arn`.\n"]
    pub fn set_scanner_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scanner_role_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBackupPlan_ScanSetting {
    type O = terrars::BlockAssignable<DataBackupPlan_ScanSetting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBackupPlan_ScanSetting {}

impl BuildDataBackupPlan_ScanSetting {
    pub fn build(self) -> DataBackupPlan_ScanSetting {
        DataBackupPlan_ScanSetting {
            malware_scanner: core::default::Default::default(),
            resource_types: core::default::Default::default(),
            scanner_role_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBackupPlan_ScanSettingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBackupPlan_ScanSettingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBackupPlan_ScanSettingRef {
        DataBackupPlan_ScanSettingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBackupPlan_ScanSettingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `malware_scanner` after provisioning.\n"]
    pub fn malware_scanner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.malware_scanner", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_types", self.base))
    }

    #[doc = "Get a reference to the value of field `scanner_role_arn` after provisioning.\n"]
    pub fn scanner_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scanner_role_arn", self.base))
    }
}
