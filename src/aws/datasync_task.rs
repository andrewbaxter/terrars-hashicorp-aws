use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DatasyncTaskData {
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
    cloudwatch_log_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    destination_location_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    source_location_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excludes: std::option::Option<std::vec::Vec<DatasyncTaskExcludesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    includes: std::option::Option<std::vec::Vec<DatasyncTaskIncludesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<std::vec::Vec<DatasyncTaskOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<std::vec::Vec<DatasyncTaskScheduleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DatasyncTaskTimeoutsEl>,
    dynamic: DatasyncTaskDynamic,
}

struct DatasyncTask_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatasyncTaskData>,
}

#[derive(Clone)]
pub struct DatasyncTask(std::rc::Rc<DatasyncTask_>);

impl DatasyncTask {
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

    #[doc = "Set the field `cloudwatch_log_group_arn`.\n"]
    pub fn set_cloudwatch_log_group_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().cloudwatch_log_group_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `excludes`.\n"]
    pub fn set_excludes(self, v: impl std::convert::Into<terrars::BlockAssignable<DatasyncTaskExcludesEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().excludes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.excludes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `includes`.\n"]
    pub fn set_includes(self, v: impl std::convert::Into<terrars::BlockAssignable<DatasyncTaskIncludesEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().includes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.includes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `options`.\n"]
    pub fn set_options(self, v: impl std::convert::Into<terrars::BlockAssignable<DatasyncTaskOptionsEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(self, v: impl std::convert::Into<terrars::BlockAssignable<DatasyncTaskScheduleEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.schedule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DatasyncTaskTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_log_group_arn` after provisioning.\n"]
    pub fn cloudwatch_log_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_log_group_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_location_arn` after provisioning.\n"]
    pub fn destination_location_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_location_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_location_arn` after provisioning.\n"]
    pub fn source_location_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_location_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> terrars::ListRef<DatasyncTaskExcludesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excludes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> terrars::ListRef<DatasyncTaskIncludesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.includes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> terrars::ListRef<DatasyncTaskOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<DatasyncTaskScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatasyncTaskTimeoutsElRef {
        DatasyncTaskTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DatasyncTask {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatasyncTask { }

impl terrars::ToListMappable for DatasyncTask {
    type O = terrars::ListRef<DatasyncTaskRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatasyncTask_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_datasync_task".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatasyncTask {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub destination_location_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source_location_arn: terrars::PrimField<std::string::String>,
}

impl BuildDatasyncTask {
    pub fn build(self, stack: &mut terrars::Stack) -> DatasyncTask {
        let out = DatasyncTask(std::rc::Rc::new(DatasyncTask_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DatasyncTaskData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
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
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatasyncTaskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatasyncTaskRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatasyncTaskRef {
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

    #[doc = "Get a reference to the value of field `cloudwatch_log_group_arn` after provisioning.\n"]
    pub fn cloudwatch_log_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_log_group_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_location_arn` after provisioning.\n"]
    pub fn destination_location_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_location_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_location_arn` after provisioning.\n"]
    pub fn source_location_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_location_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `excludes` after provisioning.\n"]
    pub fn excludes(&self) -> terrars::ListRef<DatasyncTaskExcludesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excludes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `includes` after provisioning.\n"]
    pub fn includes(&self) -> terrars::ListRef<DatasyncTaskIncludesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.includes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> terrars::ListRef<DatasyncTaskOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<DatasyncTaskScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DatasyncTaskTimeoutsElRef {
        DatasyncTaskTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DatasyncTaskExcludesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatasyncTaskExcludesEl {
    #[doc = "Set the field `filter_type`.\n"]
    pub fn set_filter_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatasyncTaskExcludesEl {
    type O = terrars::BlockAssignable<DatasyncTaskExcludesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatasyncTaskExcludesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatasyncTaskExcludesElRef {
        DatasyncTaskExcludesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncTaskExcludesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter_type` after provisioning.\n"]
    pub fn filter_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DatasyncTaskIncludesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatasyncTaskIncludesEl {
    #[doc = "Set the field `filter_type`.\n"]
    pub fn set_filter_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.filter_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatasyncTaskIncludesEl {
    type O = terrars::BlockAssignable<DatasyncTaskIncludesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatasyncTaskIncludesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatasyncTaskIncludesElRef {
        DatasyncTaskIncludesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncTaskIncludesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter_type` after provisioning.\n"]
    pub fn filter_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DatasyncTaskOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    atime: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bytes_per_second: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mtime: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overwrite_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    posix_permissions: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_deleted_files: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    preserve_devices: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_descriptor_copy_flags: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_queueing: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transfer_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uid: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verify_mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatasyncTaskOptionsEl {
    #[doc = "Set the field `atime`.\n"]
    pub fn set_atime(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.atime = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bytes_per_second`.\n"]
    pub fn set_bytes_per_second(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.bytes_per_second = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gid`.\n"]
    pub fn set_gid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_level`.\n"]
    pub fn set_log_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mtime`.\n"]
    pub fn set_mtime(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mtime = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `overwrite_mode`.\n"]
    pub fn set_overwrite_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.overwrite_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `posix_permissions`.\n"]
    pub fn set_posix_permissions(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.posix_permissions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preserve_deleted_files`.\n"]
    pub fn set_preserve_deleted_files(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.preserve_deleted_files = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `preserve_devices`.\n"]
    pub fn set_preserve_devices(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.preserve_devices = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_descriptor_copy_flags`.\n"]
    pub fn set_security_descriptor_copy_flags(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.security_descriptor_copy_flags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `task_queueing`.\n"]
    pub fn set_task_queueing(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.task_queueing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transfer_mode`.\n"]
    pub fn set_transfer_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.transfer_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `uid`.\n"]
    pub fn set_uid(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.uid = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `verify_mode`.\n"]
    pub fn set_verify_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.verify_mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatasyncTaskOptionsEl {
    type O = terrars::BlockAssignable<DatasyncTaskOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatasyncTaskOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatasyncTaskOptionsElRef {
        DatasyncTaskOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncTaskOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `atime` after provisioning.\n"]
    pub fn atime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.atime", self.base))
    }

    #[doc = "Get a reference to the value of field `bytes_per_second` after provisioning.\n"]
    pub fn bytes_per_second(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bytes_per_second", self.base))
    }

    #[doc = "Get a reference to the value of field `gid` after provisioning.\n"]
    pub fn gid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gid", self.base))
    }

    #[doc = "Get a reference to the value of field `log_level` after provisioning.\n"]
    pub fn log_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_level", self.base))
    }

    #[doc = "Get a reference to the value of field `mtime` after provisioning.\n"]
    pub fn mtime(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mtime", self.base))
    }

    #[doc = "Get a reference to the value of field `overwrite_mode` after provisioning.\n"]
    pub fn overwrite_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overwrite_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `posix_permissions` after provisioning.\n"]
    pub fn posix_permissions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.posix_permissions", self.base))
    }

    #[doc = "Get a reference to the value of field `preserve_deleted_files` after provisioning.\n"]
    pub fn preserve_deleted_files(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_deleted_files", self.base))
    }

    #[doc = "Get a reference to the value of field `preserve_devices` after provisioning.\n"]
    pub fn preserve_devices(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preserve_devices", self.base))
    }

    #[doc = "Get a reference to the value of field `security_descriptor_copy_flags` after provisioning.\n"]
    pub fn security_descriptor_copy_flags(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_descriptor_copy_flags", self.base))
    }

    #[doc = "Get a reference to the value of field `task_queueing` after provisioning.\n"]
    pub fn task_queueing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_queueing", self.base))
    }

    #[doc = "Get a reference to the value of field `transfer_mode` after provisioning.\n"]
    pub fn transfer_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transfer_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }

    #[doc = "Get a reference to the value of field `verify_mode` after provisioning.\n"]
    pub fn verify_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verify_mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DatasyncTaskScheduleEl {
    schedule_expression: terrars::PrimField<std::string::String>,
}

impl DatasyncTaskScheduleEl { }

impl terrars::ToListMappable for DatasyncTaskScheduleEl {
    type O = terrars::BlockAssignable<DatasyncTaskScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncTaskScheduleEl {
    #[doc = ""]
    pub schedule_expression: terrars::PrimField<std::string::String>,
}

impl BuildDatasyncTaskScheduleEl {
    pub fn build(self) -> DatasyncTaskScheduleEl {
        DatasyncTaskScheduleEl { schedule_expression: self.schedule_expression }
    }
}

pub struct DatasyncTaskScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatasyncTaskScheduleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatasyncTaskScheduleElRef {
        DatasyncTaskScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncTaskScheduleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `schedule_expression` after provisioning.\n"]
    pub fn schedule_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule_expression", self.base))
    }
}

#[derive(Serialize)]
pub struct DatasyncTaskTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatasyncTaskTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatasyncTaskTimeoutsEl {
    type O = terrars::BlockAssignable<DatasyncTaskTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatasyncTaskTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DatasyncTaskTimeoutsElRef {
        DatasyncTaskTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncTaskTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatasyncTaskDynamic {
    excludes: std::option::Option<terrars::DynamicBlock<DatasyncTaskExcludesEl>>,
    includes: std::option::Option<terrars::DynamicBlock<DatasyncTaskIncludesEl>>,
    options: std::option::Option<terrars::DynamicBlock<DatasyncTaskOptionsEl>>,
    schedule: std::option::Option<terrars::DynamicBlock<DatasyncTaskScheduleEl>>,
}
