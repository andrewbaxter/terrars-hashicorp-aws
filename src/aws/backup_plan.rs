use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BackupPlanData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_backup_setting: std::option::Option<std::vec::Vec<BackupPlanAdvancedBackupSettingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<BackupPlanRuleEl>>,
    dynamic: BackupPlanDynamic,
}

struct BackupPlan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BackupPlanData>,
}

#[derive(Clone)]
pub struct BackupPlan(std::rc::Rc<BackupPlan_>);

impl BackupPlan {
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

    #[doc = "Set the field `advanced_backup_setting`.\n"]
    pub fn set_advanced_backup_setting(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupPlanAdvancedBackupSettingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advanced_backup_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advanced_backup_setting = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(self, v: impl std::convert::Into<terrars::BlockAssignable<BackupPlanRuleEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

impl terrars::Referable for BackupPlan {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BackupPlan { }

impl terrars::ToListMappable for BackupPlan {
    type O = terrars::ListRef<BackupPlanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BackupPlan_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_backup_plan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBackupPlan {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBackupPlan {
    pub fn build(self, stack: &mut terrars::Stack) -> BackupPlan {
        let out = BackupPlan(std::rc::Rc::new(BackupPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BackupPlanData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                advanced_backup_setting: core::default::Default::default(),
                rule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BackupPlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupPlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BackupPlanRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BackupPlanAdvancedBackupSettingEl {
    backup_options: terrars::RecField<terrars::PrimField<std::string::String>>,
    resource_type: terrars::PrimField<std::string::String>,
}

impl BackupPlanAdvancedBackupSettingEl { }

impl terrars::ToListMappable for BackupPlanAdvancedBackupSettingEl {
    type O = terrars::BlockAssignable<BackupPlanAdvancedBackupSettingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupPlanAdvancedBackupSettingEl {
    #[doc = ""]
    pub backup_options: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub resource_type: terrars::PrimField<std::string::String>,
}

impl BuildBackupPlanAdvancedBackupSettingEl {
    pub fn build(self) -> BackupPlanAdvancedBackupSettingEl {
        BackupPlanAdvancedBackupSettingEl {
            backup_options: self.backup_options,
            resource_type: self.resource_type,
        }
    }
}

pub struct BackupPlanAdvancedBackupSettingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupPlanAdvancedBackupSettingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BackupPlanAdvancedBackupSettingElRef {
        BackupPlanAdvancedBackupSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupPlanAdvancedBackupSettingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_options` after provisioning.\n"]
    pub fn backup_options(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.backup_options", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupPlanRuleElCopyActionElLifecycleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cold_storage_after: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_after: std::option::Option<terrars::PrimField<f64>>,
}

impl BackupPlanRuleElCopyActionElLifecycleEl {
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
}

impl terrars::ToListMappable for BackupPlanRuleElCopyActionElLifecycleEl {
    type O = terrars::BlockAssignable<BackupPlanRuleElCopyActionElLifecycleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupPlanRuleElCopyActionElLifecycleEl {}

impl BuildBackupPlanRuleElCopyActionElLifecycleEl {
    pub fn build(self) -> BackupPlanRuleElCopyActionElLifecycleEl {
        BackupPlanRuleElCopyActionElLifecycleEl {
            cold_storage_after: core::default::Default::default(),
            delete_after: core::default::Default::default(),
        }
    }
}

pub struct BackupPlanRuleElCopyActionElLifecycleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupPlanRuleElCopyActionElLifecycleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BackupPlanRuleElCopyActionElLifecycleElRef {
        BackupPlanRuleElCopyActionElLifecycleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupPlanRuleElCopyActionElLifecycleElRef {
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
}

#[derive(Serialize, Default)]
struct BackupPlanRuleElCopyActionElDynamic {
    lifecycle: std::option::Option<terrars::DynamicBlock<BackupPlanRuleElCopyActionElLifecycleEl>>,
}

#[derive(Serialize)]
pub struct BackupPlanRuleElCopyActionEl {
    destination_vault_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle: std::option::Option<std::vec::Vec<BackupPlanRuleElCopyActionElLifecycleEl>>,
    dynamic: BackupPlanRuleElCopyActionElDynamic,
}

impl BackupPlanRuleElCopyActionEl {
    #[doc = "Set the field `lifecycle`.\n"]
    pub fn set_lifecycle(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupPlanRuleElCopyActionElLifecycleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lifecycle = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lifecycle = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BackupPlanRuleElCopyActionEl {
    type O = terrars::BlockAssignable<BackupPlanRuleElCopyActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupPlanRuleElCopyActionEl {
    #[doc = ""]
    pub destination_vault_arn: terrars::PrimField<std::string::String>,
}

impl BuildBackupPlanRuleElCopyActionEl {
    pub fn build(self) -> BackupPlanRuleElCopyActionEl {
        BackupPlanRuleElCopyActionEl {
            destination_vault_arn: self.destination_vault_arn,
            lifecycle: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct BackupPlanRuleElCopyActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupPlanRuleElCopyActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BackupPlanRuleElCopyActionElRef {
        BackupPlanRuleElCopyActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupPlanRuleElCopyActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_vault_arn` after provisioning.\n"]
    pub fn destination_vault_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_vault_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle` after provisioning.\n"]
    pub fn lifecycle(&self) -> terrars::ListRef<BackupPlanRuleElCopyActionElLifecycleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupPlanRuleElLifecycleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cold_storage_after: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_after: std::option::Option<terrars::PrimField<f64>>,
}

impl BackupPlanRuleElLifecycleEl {
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
}

impl terrars::ToListMappable for BackupPlanRuleElLifecycleEl {
    type O = terrars::BlockAssignable<BackupPlanRuleElLifecycleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupPlanRuleElLifecycleEl {}

impl BuildBackupPlanRuleElLifecycleEl {
    pub fn build(self) -> BackupPlanRuleElLifecycleEl {
        BackupPlanRuleElLifecycleEl {
            cold_storage_after: core::default::Default::default(),
            delete_after: core::default::Default::default(),
        }
    }
}

pub struct BackupPlanRuleElLifecycleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupPlanRuleElLifecycleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BackupPlanRuleElLifecycleElRef {
        BackupPlanRuleElLifecycleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupPlanRuleElLifecycleElRef {
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
}

#[derive(Serialize, Default)]
struct BackupPlanRuleElDynamic {
    copy_action: std::option::Option<terrars::DynamicBlock<BackupPlanRuleElCopyActionEl>>,
    lifecycle: std::option::Option<terrars::DynamicBlock<BackupPlanRuleElLifecycleEl>>,
}

#[derive(Serialize)]
pub struct BackupPlanRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    completion_window: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_continuous_backup: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recovery_point_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    rule_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_window: std::option::Option<terrars::PrimField<f64>>,
    target_vault_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_action: std::option::Option<std::vec::Vec<BackupPlanRuleElCopyActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle: std::option::Option<std::vec::Vec<BackupPlanRuleElLifecycleEl>>,
    dynamic: BackupPlanRuleElDynamic,
}

impl BackupPlanRuleEl {
    #[doc = "Set the field `completion_window`.\n"]
    pub fn set_completion_window(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.completion_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_continuous_backup`.\n"]
    pub fn set_enable_continuous_backup(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_continuous_backup = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.schedule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_window`.\n"]
    pub fn set_start_window(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.start_window = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `copy_action`.\n"]
    pub fn set_copy_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupPlanRuleElCopyActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.copy_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.copy_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lifecycle`.\n"]
    pub fn set_lifecycle(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupPlanRuleElLifecycleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lifecycle = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lifecycle = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BackupPlanRuleEl {
    type O = terrars::BlockAssignable<BackupPlanRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupPlanRuleEl {
    #[doc = ""]
    pub rule_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_vault_name: terrars::PrimField<std::string::String>,
}

impl BuildBackupPlanRuleEl {
    pub fn build(self) -> BackupPlanRuleEl {
        BackupPlanRuleEl {
            completion_window: core::default::Default::default(),
            enable_continuous_backup: core::default::Default::default(),
            recovery_point_tags: core::default::Default::default(),
            rule_name: self.rule_name,
            schedule: core::default::Default::default(),
            start_window: core::default::Default::default(),
            target_vault_name: self.target_vault_name,
            copy_action: core::default::Default::default(),
            lifecycle: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct BackupPlanRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupPlanRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BackupPlanRuleElRef {
        BackupPlanRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupPlanRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `completion_window` after provisioning.\n"]
    pub fn completion_window(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.completion_window", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_continuous_backup` after provisioning.\n"]
    pub fn enable_continuous_backup(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_continuous_backup", self.base))
    }

    #[doc = "Get a reference to the value of field `recovery_point_tags` after provisioning.\n"]
    pub fn recovery_point_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.recovery_point_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.base))
    }

    #[doc = "Get a reference to the value of field `start_window` after provisioning.\n"]
    pub fn start_window(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_window", self.base))
    }

    #[doc = "Get a reference to the value of field `target_vault_name` after provisioning.\n"]
    pub fn target_vault_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_vault_name", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle` after provisioning.\n"]
    pub fn lifecycle(&self) -> terrars::ListRef<BackupPlanRuleElLifecycleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupPlanDynamic {
    advanced_backup_setting: std::option::Option<terrars::DynamicBlock<BackupPlanAdvancedBackupSettingEl>>,
    rule: std::option::Option<terrars::DynamicBlock<BackupPlanRuleEl>>,
}
