use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DmsReplicationTaskData {
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
    cdc_start_position: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdc_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    migration_type: terrars::PrimField<std::string::String>,
    replication_instance_arn: terrars::PrimField<std::string::String>,
    replication_task_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_task_settings: std::option::Option<terrars::PrimField<std::string::String>>,
    source_endpoint_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_replication_task: std::option::Option<terrars::PrimField<bool>>,
    table_mappings: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    target_endpoint_arn: terrars::PrimField<std::string::String>,
}

struct DmsReplicationTask_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DmsReplicationTaskData>,
}

#[derive(Clone)]
pub struct DmsReplicationTask(std::rc::Rc<DmsReplicationTask_>);

impl DmsReplicationTask {
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

    #[doc = "Set the field `cdc_start_position`.\n"]
    pub fn set_cdc_start_position(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cdc_start_position = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cdc_start_time`.\n"]
    pub fn set_cdc_start_time(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cdc_start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replication_task_settings`.\n"]
    pub fn set_replication_task_settings(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().replication_task_settings = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_replication_task`.\n"]
    pub fn set_start_replication_task(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().start_replication_task = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `cdc_start_position` after provisioning.\n"]
    pub fn cdc_start_position(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdc_start_position", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_start_time` after provisioning.\n"]
    pub fn cdc_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdc_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `migration_type` after provisioning.\n"]
    pub fn migration_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.migration_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_instance_arn` after provisioning.\n"]
    pub fn replication_instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_instance_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_task_arn` after provisioning.\n"]
    pub fn replication_task_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_task_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_task_id` after provisioning.\n"]
    pub fn replication_task_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_task_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_task_settings` after provisioning.\n"]
    pub fn replication_task_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_task_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_endpoint_arn` after provisioning.\n"]
    pub fn source_endpoint_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_endpoint_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_replication_task` after provisioning.\n"]
    pub fn start_replication_task(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_replication_task", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_mappings` after provisioning.\n"]
    pub fn table_mappings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_mappings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_endpoint_arn` after provisioning.\n"]
    pub fn target_endpoint_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_endpoint_arn", self.extract_ref()))
    }
}

impl terrars::Referable for DmsReplicationTask {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DmsReplicationTask { }

impl terrars::ToListMappable for DmsReplicationTask {
    type O = terrars::ListRef<DmsReplicationTaskRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DmsReplicationTask_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_dms_replication_task".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDmsReplicationTask {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub migration_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub replication_instance_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub replication_task_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source_endpoint_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub table_mappings: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_endpoint_arn: terrars::PrimField<std::string::String>,
}

impl BuildDmsReplicationTask {
    pub fn build(self, stack: &mut terrars::Stack) -> DmsReplicationTask {
        let out = DmsReplicationTask(std::rc::Rc::new(DmsReplicationTask_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DmsReplicationTaskData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cdc_start_position: core::default::Default::default(),
                cdc_start_time: core::default::Default::default(),
                id: core::default::Default::default(),
                migration_type: self.migration_type,
                replication_instance_arn: self.replication_instance_arn,
                replication_task_id: self.replication_task_id,
                replication_task_settings: core::default::Default::default(),
                source_endpoint_arn: self.source_endpoint_arn,
                start_replication_task: core::default::Default::default(),
                table_mappings: self.table_mappings,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                target_endpoint_arn: self.target_endpoint_arn,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DmsReplicationTaskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DmsReplicationTaskRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DmsReplicationTaskRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cdc_start_position` after provisioning.\n"]
    pub fn cdc_start_position(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdc_start_position", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cdc_start_time` after provisioning.\n"]
    pub fn cdc_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdc_start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `migration_type` after provisioning.\n"]
    pub fn migration_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.migration_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_instance_arn` after provisioning.\n"]
    pub fn replication_instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_instance_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_task_arn` after provisioning.\n"]
    pub fn replication_task_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_task_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_task_id` after provisioning.\n"]
    pub fn replication_task_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_task_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_task_settings` after provisioning.\n"]
    pub fn replication_task_settings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.replication_task_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_endpoint_arn` after provisioning.\n"]
    pub fn source_endpoint_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_endpoint_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_replication_task` after provisioning.\n"]
    pub fn start_replication_task(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_replication_task", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `table_mappings` after provisioning.\n"]
    pub fn table_mappings(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.table_mappings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_endpoint_arn` after provisioning.\n"]
    pub fn target_endpoint_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_endpoint_arn", self.extract_ref()))
    }
}
