use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BackupReportPlanData {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    report_delivery_channel: std::option::Option<std::vec::Vec<BackupReportPlanReportDeliveryChannelEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    report_setting: std::option::Option<std::vec::Vec<BackupReportPlanReportSettingEl>>,
    dynamic: BackupReportPlanDynamic,
}

struct BackupReportPlan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BackupReportPlanData>,
}

#[derive(Clone)]
pub struct BackupReportPlan(std::rc::Rc<BackupReportPlan_>);

impl BackupReportPlan {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `report_delivery_channel`.\n"]
    pub fn set_report_delivery_channel(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupReportPlanReportDeliveryChannelEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().report_delivery_channel = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.report_delivery_channel = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `report_setting`.\n"]
    pub fn set_report_setting(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupReportPlanReportSettingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().report_setting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.report_setting = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_status` after provisioning.\n"]
    pub fn deployment_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `report_delivery_channel` after provisioning.\n"]
    pub fn report_delivery_channel(&self) -> terrars::ListRef<BackupReportPlanReportDeliveryChannelElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.report_delivery_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_setting` after provisioning.\n"]
    pub fn report_setting(&self) -> terrars::ListRef<BackupReportPlanReportSettingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.report_setting", self.extract_ref()))
    }
}

impl terrars::Referable for BackupReportPlan {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BackupReportPlan { }

impl terrars::ToListMappable for BackupReportPlan {
    type O = terrars::ListRef<BackupReportPlanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BackupReportPlan_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_backup_report_plan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBackupReportPlan {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBackupReportPlan {
    pub fn build(self, stack: &mut terrars::Stack) -> BackupReportPlan {
        let out = BackupReportPlan(std::rc::Rc::new(BackupReportPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BackupReportPlanData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                report_delivery_channel: core::default::Default::default(),
                report_setting: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BackupReportPlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupReportPlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BackupReportPlanRef {
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

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_status` after provisioning.\n"]
    pub fn deployment_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `report_delivery_channel` after provisioning.\n"]
    pub fn report_delivery_channel(&self) -> terrars::ListRef<BackupReportPlanReportDeliveryChannelElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.report_delivery_channel", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `report_setting` after provisioning.\n"]
    pub fn report_setting(&self) -> terrars::ListRef<BackupReportPlanReportSettingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.report_setting", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BackupReportPlanReportDeliveryChannelEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    formats: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    s3_bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_key_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BackupReportPlanReportDeliveryChannelEl {
    #[doc = "Set the field `formats`.\n"]
    pub fn set_formats(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.formats = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_key_prefix`.\n"]
    pub fn set_s3_key_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_key_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupReportPlanReportDeliveryChannelEl {
    type O = terrars::BlockAssignable<BackupReportPlanReportDeliveryChannelEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupReportPlanReportDeliveryChannelEl {
    #[doc = ""]
    pub s3_bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildBackupReportPlanReportDeliveryChannelEl {
    pub fn build(self) -> BackupReportPlanReportDeliveryChannelEl {
        BackupReportPlanReportDeliveryChannelEl {
            formats: core::default::Default::default(),
            s3_bucket_name: self.s3_bucket_name,
            s3_key_prefix: core::default::Default::default(),
        }
    }
}

pub struct BackupReportPlanReportDeliveryChannelElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupReportPlanReportDeliveryChannelElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BackupReportPlanReportDeliveryChannelElRef {
        BackupReportPlanReportDeliveryChannelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupReportPlanReportDeliveryChannelElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `formats` after provisioning.\n"]
    pub fn formats(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.formats", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_key_prefix` after provisioning.\n"]
    pub fn s3_key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_key_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupReportPlanReportSettingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    framework_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_of_frameworks: std::option::Option<terrars::PrimField<f64>>,
    report_template: terrars::PrimField<std::string::String>,
}

impl BackupReportPlanReportSettingEl {
    #[doc = "Set the field `framework_arns`.\n"]
    pub fn set_framework_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.framework_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_frameworks`.\n"]
    pub fn set_number_of_frameworks(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.number_of_frameworks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BackupReportPlanReportSettingEl {
    type O = terrars::BlockAssignable<BackupReportPlanReportSettingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupReportPlanReportSettingEl {
    #[doc = ""]
    pub report_template: terrars::PrimField<std::string::String>,
}

impl BuildBackupReportPlanReportSettingEl {
    pub fn build(self) -> BackupReportPlanReportSettingEl {
        BackupReportPlanReportSettingEl {
            framework_arns: core::default::Default::default(),
            number_of_frameworks: core::default::Default::default(),
            report_template: self.report_template,
        }
    }
}

pub struct BackupReportPlanReportSettingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupReportPlanReportSettingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BackupReportPlanReportSettingElRef {
        BackupReportPlanReportSettingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupReportPlanReportSettingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `framework_arns` after provisioning.\n"]
    pub fn framework_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.framework_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `number_of_frameworks` after provisioning.\n"]
    pub fn number_of_frameworks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_frameworks", self.base))
    }

    #[doc = "Get a reference to the value of field `report_template` after provisioning.\n"]
    pub fn report_template(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.report_template", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupReportPlanDynamic {
    report_delivery_channel: std::option::Option<terrars::DynamicBlock<BackupReportPlanReportDeliveryChannelEl>>,
    report_setting: std::option::Option<terrars::DynamicBlock<BackupReportPlanReportSettingEl>>,
}
