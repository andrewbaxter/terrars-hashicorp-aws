use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Kinesisanalyticsv2ApplicationSnapshotData {
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
    application_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    snapshot_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<Kinesisanalyticsv2ApplicationSnapshotTimeoutsEl>,
}

struct Kinesisanalyticsv2ApplicationSnapshot_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Kinesisanalyticsv2ApplicationSnapshotData>,
}

#[derive(Clone)]
pub struct Kinesisanalyticsv2ApplicationSnapshot(std::rc::Rc<Kinesisanalyticsv2ApplicationSnapshot_>);

impl Kinesisanalyticsv2ApplicationSnapshot {
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<Kinesisanalyticsv2ApplicationSnapshotTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `application_name` after provisioning.\n"]
    pub fn application_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_version_id` after provisioning.\n"]
    pub fn application_version_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_creation_timestamp` after provisioning.\n"]
    pub fn snapshot_creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_creation_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_name` after provisioning.\n"]
    pub fn snapshot_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Kinesisanalyticsv2ApplicationSnapshotTimeoutsElRef {
        Kinesisanalyticsv2ApplicationSnapshotTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for Kinesisanalyticsv2ApplicationSnapshot {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Kinesisanalyticsv2ApplicationSnapshot { }

impl terrars::ToListMappable for Kinesisanalyticsv2ApplicationSnapshot {
    type O = terrars::ListRef<Kinesisanalyticsv2ApplicationSnapshotRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Kinesisanalyticsv2ApplicationSnapshot_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_kinesisanalyticsv2_application_snapshot".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKinesisanalyticsv2ApplicationSnapshot {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub application_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub snapshot_name: terrars::PrimField<std::string::String>,
}

impl BuildKinesisanalyticsv2ApplicationSnapshot {
    pub fn build(self, stack: &mut terrars::Stack) -> Kinesisanalyticsv2ApplicationSnapshot {
        let out = Kinesisanalyticsv2ApplicationSnapshot(std::rc::Rc::new(Kinesisanalyticsv2ApplicationSnapshot_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Kinesisanalyticsv2ApplicationSnapshotData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                application_name: self.application_name,
                id: core::default::Default::default(),
                snapshot_name: self.snapshot_name,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Kinesisanalyticsv2ApplicationSnapshotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2ApplicationSnapshotRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Kinesisanalyticsv2ApplicationSnapshotRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_name` after provisioning.\n"]
    pub fn application_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `application_version_id` after provisioning.\n"]
    pub fn application_version_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_version_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_creation_timestamp` after provisioning.\n"]
    pub fn snapshot_creation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_creation_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `snapshot_name` after provisioning.\n"]
    pub fn snapshot_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> Kinesisanalyticsv2ApplicationSnapshotTimeoutsElRef {
        Kinesisanalyticsv2ApplicationSnapshotTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct Kinesisanalyticsv2ApplicationSnapshotTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Kinesisanalyticsv2ApplicationSnapshotTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Kinesisanalyticsv2ApplicationSnapshotTimeoutsEl {
    type O = terrars::BlockAssignable<Kinesisanalyticsv2ApplicationSnapshotTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKinesisanalyticsv2ApplicationSnapshotTimeoutsEl {}

impl BuildKinesisanalyticsv2ApplicationSnapshotTimeoutsEl {
    pub fn build(self) -> Kinesisanalyticsv2ApplicationSnapshotTimeoutsEl {
        Kinesisanalyticsv2ApplicationSnapshotTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct Kinesisanalyticsv2ApplicationSnapshotTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Kinesisanalyticsv2ApplicationSnapshotTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Kinesisanalyticsv2ApplicationSnapshotTimeoutsElRef {
        Kinesisanalyticsv2ApplicationSnapshotTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Kinesisanalyticsv2ApplicationSnapshotTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
