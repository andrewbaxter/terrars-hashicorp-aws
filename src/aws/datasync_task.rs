use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DatasyncTaskData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_log_group_arn: Option<PrimField<String>>,
    destination_location_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    source_location_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excludes: Option<Vec<DatasyncTaskExcludesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    includes: Option<Vec<DatasyncTaskIncludesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Vec<DatasyncTaskOptionsEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schedule: Option<Vec<DatasyncTaskScheduleEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DatasyncTaskTimeoutsEl>,
    dynamic: DatasyncTaskDynamic,
}

struct DatasyncTask_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DatasyncTaskData>,
}

#[derive(Clone)]
pub struct DatasyncTask(Rc<DatasyncTask_>);

impl DatasyncTask {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `cloudwatch_log_group_arn`.\n"]
    pub fn set_cloudwatch_log_group_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().cloudwatch_log_group_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `excludes`.\n"]
    pub fn set_excludes(self, v: impl Into<BlockAssignable<DatasyncTaskExcludesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().excludes = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.excludes = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `includes`.\n"]
    pub fn set_includes(self, v: impl Into<BlockAssignable<DatasyncTaskIncludesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().includes = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.includes = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `options`.\n"]
    pub fn set_options(self, v: impl Into<BlockAssignable<DatasyncTaskOptionsEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().options = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.options = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `schedule`.\n"]
    pub fn set_schedule(self, v: impl Into<BlockAssignable<DatasyncTaskScheduleEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DatasyncTaskTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloudwatch_log_group_arn` after provisioning.\n"]
    pub fn cloudwatch_log_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_log_group_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `destination_location_arn` after provisioning.\n"]
    pub fn destination_location_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_location_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_location_arn` after provisioning.\n"]
    pub fn source_location_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_location_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> ListRef<DatasyncTaskExcludesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.excludes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> ListRef<DatasyncTaskIncludesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.includes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> ListRef<DatasyncTaskOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<DatasyncTaskScheduleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatasyncTaskTimeoutsElRef {
        DatasyncTaskTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Resource for DatasyncTask {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DatasyncTask {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for DatasyncTask {
    type O = ListRef<DatasyncTaskRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for DatasyncTask_ {
    fn extract_resource_type(&self) -> String {
        "aws_datasync_task".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatasyncTask {
    pub tf_id: String,
    #[doc= ""]
    pub destination_location_arn: PrimField<String>,
    #[doc= ""]
    pub source_location_arn: PrimField<String>,
}

impl BuildDatasyncTask {
    pub fn build(self, stack: &mut Stack) -> DatasyncTask {
        let out = DatasyncTask(Rc::new(DatasyncTask_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DatasyncTaskData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                cloudwatch_log_group_arn: core::default::Default::default(),
                destination_location_arn: self.destination_location_arn,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                source_location_arn: self.source_location_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                excludes: core::default::Default::default(),
                includes: core::default::Default::default(),
                options: core::default::Default::default(),
                schedule: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatasyncTaskRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncTaskRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatasyncTaskRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloudwatch_log_group_arn` after provisioning.\n"]
    pub fn cloudwatch_log_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_log_group_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `destination_location_arn` after provisioning.\n"]
    pub fn destination_location_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination_location_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source_location_arn` after provisioning.\n"]
    pub fn source_location_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_location_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> ListRef<DatasyncTaskExcludesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.excludes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> ListRef<DatasyncTaskIncludesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.includes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> ListRef<DatasyncTaskOptionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> ListRef<DatasyncTaskScheduleElRef> {
        ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatasyncTaskTimeoutsElRef {
        DatasyncTaskTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DatasyncTaskExcludesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DatasyncTaskExcludesEl {
    #[doc= "Set the field `filter_type`.\n"]
    pub fn set_filter_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_type = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DatasyncTaskExcludesEl {
    type O = BlockAssignable<DatasyncTaskExcludesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncTaskExcludesEl {}

impl BuildDatasyncTaskExcludesEl {
    pub fn build(self) -> DatasyncTaskExcludesEl {
        DatasyncTaskExcludesEl {
            filter_type: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DatasyncTaskExcludesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncTaskExcludesElRef {
    fn new(shared: StackShared, base: String) -> DatasyncTaskExcludesElRef {
        DatasyncTaskExcludesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncTaskExcludesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `filter_type` after provisioning.\n"]
    pub fn filter_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter_type", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DatasyncTaskIncludesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl DatasyncTaskIncludesEl {
    #[doc= "Set the field `filter_type`.\n"]
    pub fn set_filter_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.filter_type = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for DatasyncTaskIncludesEl {
    type O = BlockAssignable<DatasyncTaskIncludesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncTaskIncludesEl {}

impl BuildDatasyncTaskIncludesEl {
    pub fn build(self) -> DatasyncTaskIncludesEl {
        DatasyncTaskIncludesEl {
            filter_type: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct DatasyncTaskIncludesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncTaskIncludesElRef {
    fn new(shared: StackShared, base: String) -> DatasyncTaskIncludesElRef {
        DatasyncTaskIncludesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncTaskIncludesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `filter_type` after provisioning.\n"]
    pub fn filter_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filter_type", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DatasyncTaskOptionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    atime: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bytes_per_second: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mtime: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overwrite_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    posix_permissions: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve_deleted_files: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preserve_devices: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    security_descriptor_copy_flags: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    task_queueing: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verify_mode: Option<PrimField<String>>,
}

impl DatasyncTaskOptionsEl {
    #[doc= "Set the field `atime`.\n"]
    pub fn set_atime(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.atime = Some(v.into());
        self
    }

    #[doc= "Set the field `bytes_per_second`.\n"]
    pub fn set_bytes_per_second(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.bytes_per_second = Some(v.into());
        self
    }

    #[doc= "Set the field `gid`.\n"]
    pub fn set_gid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.gid = Some(v.into());
        self
    }

    #[doc= "Set the field `log_level`.\n"]
    pub fn set_log_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_level = Some(v.into());
        self
    }

    #[doc= "Set the field `mtime`.\n"]
    pub fn set_mtime(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mtime = Some(v.into());
        self
    }

    #[doc= "Set the field `overwrite_mode`.\n"]
    pub fn set_overwrite_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.overwrite_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `posix_permissions`.\n"]
    pub fn set_posix_permissions(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.posix_permissions = Some(v.into());
        self
    }

    #[doc= "Set the field `preserve_deleted_files`.\n"]
    pub fn set_preserve_deleted_files(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.preserve_deleted_files = Some(v.into());
        self
    }

    #[doc= "Set the field `preserve_devices`.\n"]
    pub fn set_preserve_devices(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.preserve_devices = Some(v.into());
        self
    }

    #[doc= "Set the field `security_descriptor_copy_flags`.\n"]
    pub fn set_security_descriptor_copy_flags(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.security_descriptor_copy_flags = Some(v.into());
        self
    }

    #[doc= "Set the field `task_queueing`.\n"]
    pub fn set_task_queueing(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.task_queueing = Some(v.into());
        self
    }

    #[doc= "Set the field `transfer_mode`.\n"]
    pub fn set_transfer_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.transfer_mode = Some(v.into());
        self
    }

    #[doc= "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.uid = Some(v.into());
        self
    }

    #[doc= "Set the field `verify_mode`.\n"]
    pub fn set_verify_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.verify_mode = Some(v.into());
        self
    }
}

impl ToListMappable for DatasyncTaskOptionsEl {
    type O = BlockAssignable<DatasyncTaskOptionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncTaskOptionsEl {}

impl BuildDatasyncTaskOptionsEl {
    pub fn build(self) -> DatasyncTaskOptionsEl {
        DatasyncTaskOptionsEl {
            atime: core::default::Default::default(),
            bytes_per_second: core::default::Default::default(),
            gid: core::default::Default::default(),
            log_level: core::default::Default::default(),
            mtime: core::default::Default::default(),
            overwrite_mode: core::default::Default::default(),
            posix_permissions: core::default::Default::default(),
            preserve_deleted_files: core::default::Default::default(),
            preserve_devices: core::default::Default::default(),
            security_descriptor_copy_flags: core::default::Default::default(),
            task_queueing: core::default::Default::default(),
            transfer_mode: core::default::Default::default(),
            uid: core::default::Default::default(),
            verify_mode: core::default::Default::default(),
        }
    }
}

pub struct DatasyncTaskOptionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncTaskOptionsElRef {
    fn new(shared: StackShared, base: String) -> DatasyncTaskOptionsElRef {
        DatasyncTaskOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncTaskOptionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `atime` after provisioning.\n"]
    pub fn atime(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.atime", self.base))
    }

    #[doc= "Get a reference to the value of field `bytes_per_second` after provisioning.\n"]
    pub fn bytes_per_second(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.bytes_per_second", self.base))
    }

    #[doc= "Get a reference to the value of field `gid` after provisioning.\n"]
    pub fn gid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.gid", self.base))
    }

    #[doc= "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }

    #[doc= "Get a reference to the value of field `mtime` after provisioning.\n"]
    pub fn mtime(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mtime", self.base))
    }

    #[doc= "Get a reference to the value of field `overwrite_mode` after provisioning.\n"]
    pub fn overwrite_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.overwrite_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `posix_permissions` after provisioning.\n"]
    pub fn posix_permissions(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.posix_permissions", self.base))
    }

    #[doc= "Get a reference to the value of field `preserve_deleted_files` after provisioning.\n"]
    pub fn preserve_deleted_files(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_deleted_files", self.base))
    }

    #[doc= "Get a reference to the value of field `preserve_devices` after provisioning.\n"]
    pub fn preserve_devices(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.preserve_devices", self.base))
    }

    #[doc= "Get a reference to the value of field `security_descriptor_copy_flags` after provisioning.\n"]
    pub fn security_descriptor_copy_flags(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.security_descriptor_copy_flags", self.base))
    }

    #[doc= "Get a reference to the value of field `task_queueing` after provisioning.\n"]
    pub fn task_queueing(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.task_queueing", self.base))
    }

    #[doc= "Get a reference to the value of field `transfer_mode` after provisioning.\n"]
    pub fn transfer_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.transfer_mode", self.base))
    }

    #[doc= "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }

    #[doc= "Get a reference to the value of field `verify_mode` after provisioning.\n"]
    pub fn verify_mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.verify_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DatasyncTaskScheduleEl {
    schedule_expression: PrimField<String>,
}

impl DatasyncTaskScheduleEl { }

impl ToListMappable for DatasyncTaskScheduleEl {
    type O = BlockAssignable<DatasyncTaskScheduleEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncTaskScheduleEl {
    #[doc= ""]
    pub schedule_expression: PrimField<String>,
}

impl BuildDatasyncTaskScheduleEl {
    pub fn build(self) -> DatasyncTaskScheduleEl {
        DatasyncTaskScheduleEl { schedule_expression: self.schedule_expression }
    }
}

pub struct DatasyncTaskScheduleElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncTaskScheduleElRef {
    fn new(shared: StackShared, base: String) -> DatasyncTaskScheduleElRef {
        DatasyncTaskScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncTaskScheduleElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.base))
    }
}

#[derive(Serialize)]
pub struct DatasyncTaskTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<PrimField<String>>,
}

impl DatasyncTaskTimeoutsEl {
    #[doc= "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.create = Some(v.into());
        self
    }
}

impl ToListMappable for DatasyncTaskTimeoutsEl {
    type O = BlockAssignable<DatasyncTaskTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncTaskTimeoutsEl {}

impl BuildDatasyncTaskTimeoutsEl {
    pub fn build(self) -> DatasyncTaskTimeoutsEl {
        DatasyncTaskTimeoutsEl { create: core::default::Default::default() }
    }
}

pub struct DatasyncTaskTimeoutsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DatasyncTaskTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DatasyncTaskTimeoutsElRef {
        DatasyncTaskTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncTaskTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatasyncTaskDynamic {
    excludes: Option<DynamicBlock<DatasyncTaskExcludesEl>>,
    includes: Option<DynamicBlock<DatasyncTaskIncludesEl>>,
    options: Option<DynamicBlock<DatasyncTaskOptionsEl>>,
    schedule: Option<DynamicBlock<DatasyncTaskScheduleEl>>,
}
