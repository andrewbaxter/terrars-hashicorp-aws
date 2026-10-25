#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BackupRestoreTestingSelectionData_ {
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
    iam_role_arn: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protected_resource_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    protected_resource_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    restore_metadata_overrides: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    restore_testing_plan_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation_window_hours: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protected_resource_conditions: std
    ::option
    ::Option<std::vec::Vec<BackupRestoreTestingSelection_ProtectedResourceConditions>>,
    dynamic: BackupRestoreTestingSelectionDynamic,
}

struct BackupRestoreTestingSelection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BackupRestoreTestingSelectionData_>,
}

#[derive(Clone)]
pub struct BackupRestoreTestingSelection(std::rc::Rc<BackupRestoreTestingSelection_>);

impl BackupRestoreTestingSelection {
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

    #[doc = "Set the field `protected_resource_arns`.\n"]
    pub fn set_protected_resource_arns(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().protected_resource_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `restore_metadata_overrides`.\n"]
    pub fn set_restore_metadata_overrides(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().restore_metadata_overrides = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `validation_window_hours`.\n"]
    pub fn set_validation_window_hours(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().validation_window_hours = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protected_resource_conditions`.\n"]
    pub fn set_protected_resource_conditions(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BackupRestoreTestingSelection_ProtectedResourceConditions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().protected_resource_conditions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.protected_resource_conditions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protected_resource_arns` after provisioning.\n"]
    pub fn protected_resource_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.protected_resource_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protected_resource_type` after provisioning.\n"]
    pub fn protected_resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protected_resource_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_metadata_overrides` after provisioning.\n"]
    pub fn restore_metadata_overrides(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.restore_metadata_overrides", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_testing_plan_name` after provisioning.\n"]
    pub fn restore_testing_plan_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restore_testing_plan_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_window_hours` after provisioning.\n"]
    pub fn validation_window_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_window_hours", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protected_resource_conditions` after provisioning.\n"]
    pub fn protected_resource_conditions(
        &self,
    ) -> terrars::ListRef<BackupRestoreTestingSelection_ProtectedResourceConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protected_resource_conditions", self.extract_ref()))
    }
}

impl terrars::Referable for BackupRestoreTestingSelection {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BackupRestoreTestingSelection { }

impl terrars::ToListMappable for BackupRestoreTestingSelection {
    type O = terrars::ListRef<BackupRestoreTestingSelectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BackupRestoreTestingSelection_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_backup_restore_testing_selection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBackupRestoreTestingSelection {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub iam_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub protected_resource_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub restore_testing_plan_name: terrars::PrimField<std::string::String>,
}

impl BuildBackupRestoreTestingSelection {
    pub fn build(self, stack: &mut terrars::Stack) -> BackupRestoreTestingSelection {
        let out = BackupRestoreTestingSelection(std::rc::Rc::new(BackupRestoreTestingSelection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BackupRestoreTestingSelectionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                iam_role_arn: self.iam_role_arn,
                name: self.name,
                protected_resource_arns: core::default::Default::default(),
                protected_resource_type: self.protected_resource_type,
                region: core::default::Default::default(),
                restore_metadata_overrides: core::default::Default::default(),
                restore_testing_plan_name: self.restore_testing_plan_name,
                validation_window_hours: core::default::Default::default(),
                protected_resource_conditions: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BackupRestoreTestingSelectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupRestoreTestingSelectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BackupRestoreTestingSelectionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protected_resource_arns` after provisioning.\n"]
    pub fn protected_resource_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.protected_resource_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protected_resource_type` after provisioning.\n"]
    pub fn protected_resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protected_resource_type", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_metadata_overrides` after provisioning.\n"]
    pub fn restore_metadata_overrides(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.restore_metadata_overrides", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `restore_testing_plan_name` after provisioning.\n"]
    pub fn restore_testing_plan_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.restore_testing_plan_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_window_hours` after provisioning.\n"]
    pub fn validation_window_hours(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_window_hours", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protected_resource_conditions` after provisioning.\n"]
    pub fn protected_resource_conditions(
        &self,
    ) -> terrars::ListRef<BackupRestoreTestingSelection_ProtectedResourceConditionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protected_resource_conditions", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BackupRestoreTestingSelectionDynamic {
    protected_resource_conditions: std
    ::option
    ::Option<terrars::DynamicBlock<BackupRestoreTestingSelection_ProtectedResourceConditions>>,
}

#[derive(Serialize)]
pub struct BackupRestoreTestingSelection_ProtectedResourceConditions_StringEquals {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl BackupRestoreTestingSelection_ProtectedResourceConditions_StringEquals { }

impl terrars::ToListMappable for BackupRestoreTestingSelection_ProtectedResourceConditions_StringEquals {
    type O = terrars::BlockAssignable<BackupRestoreTestingSelection_ProtectedResourceConditions_StringEquals>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupRestoreTestingSelection_ProtectedResourceConditions_StringEquals {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildBackupRestoreTestingSelection_ProtectedResourceConditions_StringEquals {
    pub fn build(self) -> BackupRestoreTestingSelection_ProtectedResourceConditions_StringEquals {
        BackupRestoreTestingSelection_ProtectedResourceConditions_StringEquals {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct BackupRestoreTestingSelection_ProtectedResourceConditions_StringEqualsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupRestoreTestingSelection_ProtectedResourceConditions_StringEqualsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupRestoreTestingSelection_ProtectedResourceConditions_StringEqualsRef {
        BackupRestoreTestingSelection_ProtectedResourceConditions_StringEqualsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupRestoreTestingSelection_ProtectedResourceConditions_StringEqualsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupRestoreTestingSelection_ProtectedResourceConditions_StringNotEquals {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl BackupRestoreTestingSelection_ProtectedResourceConditions_StringNotEquals { }

impl terrars::ToListMappable for BackupRestoreTestingSelection_ProtectedResourceConditions_StringNotEquals {
    type O = terrars::BlockAssignable<BackupRestoreTestingSelection_ProtectedResourceConditions_StringNotEquals>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupRestoreTestingSelection_ProtectedResourceConditions_StringNotEquals {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildBackupRestoreTestingSelection_ProtectedResourceConditions_StringNotEquals {
    pub fn build(self) -> BackupRestoreTestingSelection_ProtectedResourceConditions_StringNotEquals {
        BackupRestoreTestingSelection_ProtectedResourceConditions_StringNotEquals {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct BackupRestoreTestingSelection_ProtectedResourceConditions_StringNotEqualsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupRestoreTestingSelection_ProtectedResourceConditions_StringNotEqualsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupRestoreTestingSelection_ProtectedResourceConditions_StringNotEqualsRef {
        BackupRestoreTestingSelection_ProtectedResourceConditions_StringNotEqualsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupRestoreTestingSelection_ProtectedResourceConditions_StringNotEqualsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupRestoreTestingSelection_ProtectedResourceConditionsDynamic {
    string_equals: std
    ::option
    ::Option<terrars::DynamicBlock<BackupRestoreTestingSelection_ProtectedResourceConditions_StringEquals>>,
    string_not_equals: std
    ::option
    ::Option<terrars::DynamicBlock<BackupRestoreTestingSelection_ProtectedResourceConditions_StringNotEquals>>,
}

#[derive(Serialize)]
pub struct BackupRestoreTestingSelection_ProtectedResourceConditions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_equals: std
    ::option
    ::Option<std::vec::Vec<BackupRestoreTestingSelection_ProtectedResourceConditions_StringEquals>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_not_equals: std
    ::option
    ::Option<std::vec::Vec<BackupRestoreTestingSelection_ProtectedResourceConditions_StringNotEquals>>,
    dynamic: BackupRestoreTestingSelection_ProtectedResourceConditionsDynamic,
}

impl BackupRestoreTestingSelection_ProtectedResourceConditions {
    #[doc = "Set the field `string_equals`.\n"]
    pub fn set_string_equals(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BackupRestoreTestingSelection_ProtectedResourceConditions_StringEquals>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_equals = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_equals = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `string_not_equals`.\n"]
    pub fn set_string_not_equals(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BackupRestoreTestingSelection_ProtectedResourceConditions_StringNotEquals>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_not_equals = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_not_equals = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BackupRestoreTestingSelection_ProtectedResourceConditions {
    type O = terrars::BlockAssignable<BackupRestoreTestingSelection_ProtectedResourceConditions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupRestoreTestingSelection_ProtectedResourceConditions {}

impl BuildBackupRestoreTestingSelection_ProtectedResourceConditions {
    pub fn build(self) -> BackupRestoreTestingSelection_ProtectedResourceConditions {
        BackupRestoreTestingSelection_ProtectedResourceConditions {
            string_equals: core::default::Default::default(),
            string_not_equals: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BackupRestoreTestingSelection_ProtectedResourceConditionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupRestoreTestingSelection_ProtectedResourceConditionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupRestoreTestingSelection_ProtectedResourceConditionsRef {
        BackupRestoreTestingSelection_ProtectedResourceConditionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupRestoreTestingSelection_ProtectedResourceConditionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `string_equals` after provisioning.\n"]
    pub fn string_equals(
        &self,
    ) -> terrars::ListRef<BackupRestoreTestingSelection_ProtectedResourceConditions_StringEqualsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_equals", self.base))
    }

    #[doc = "Get a reference to the value of field `string_not_equals` after provisioning.\n"]
    pub fn string_not_equals(
        &self,
    ) -> terrars::ListRef<BackupRestoreTestingSelection_ProtectedResourceConditions_StringNotEqualsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_not_equals", self.base))
    }
}
