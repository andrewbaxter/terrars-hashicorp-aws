use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBatchJobQueueData {
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
}

struct DataBatchJobQueue_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBatchJobQueueData>,
}

#[derive(Clone)]
pub struct DataBatchJobQueue(Rc<DataBatchJobQueue_>);

impl DataBatchJobQueue {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `compute_environment_order` after provisioning.\n"]
    pub fn compute_environment_order(&self) -> ListRef<DataBatchJobQueueComputeEnvironmentOrderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.compute_environment_order", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scheduling_policy_arn` after provisioning.\n"]
    pub fn scheduling_policy_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scheduling_policy_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl Datasource for DataBatchJobQueue {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for DataBatchJobQueue {
    type O = ListRef<DataBatchJobQueueRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataBatchJobQueue_ {
    fn extract_datasource_type(&self) -> String {
        "aws_batch_job_queue".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBatchJobQueue {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildDataBatchJobQueue {
    pub fn build(self, stack: &mut Stack) -> DataBatchJobQueue {
        let out = DataBatchJobQueue(Rc::new(DataBatchJobQueue_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBatchJobQueueData {
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBatchJobQueueRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBatchJobQueueRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBatchJobQueueRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `compute_environment_order` after provisioning.\n"]
    pub fn compute_environment_order(&self) -> ListRef<DataBatchJobQueueComputeEnvironmentOrderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.compute_environment_order", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `scheduling_policy_arn` after provisioning.\n"]
    pub fn scheduling_policy_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.scheduling_policy_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBatchJobQueueComputeEnvironmentOrderEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    compute_environment: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<PrimField<f64>>,
}

impl DataBatchJobQueueComputeEnvironmentOrderEl {
    #[doc= "Set the field `compute_environment`.\n"]
    pub fn set_compute_environment(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.compute_environment = Some(v.into());
        self
    }

    #[doc= "Set the field `order`.\n"]
    pub fn set_order(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.order = Some(v.into());
        self
    }
}

impl ToListMappable for DataBatchJobQueueComputeEnvironmentOrderEl {
    type O = BlockAssignable<DataBatchJobQueueComputeEnvironmentOrderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchJobQueueComputeEnvironmentOrderEl {}

impl BuildDataBatchJobQueueComputeEnvironmentOrderEl {
    pub fn build(self) -> DataBatchJobQueueComputeEnvironmentOrderEl {
        DataBatchJobQueueComputeEnvironmentOrderEl {
            compute_environment: core::default::Default::default(),
            order: core::default::Default::default(),
        }
    }
}

pub struct DataBatchJobQueueComputeEnvironmentOrderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBatchJobQueueComputeEnvironmentOrderElRef {
    fn new(shared: StackShared, base: String) -> DataBatchJobQueueComputeEnvironmentOrderElRef {
        DataBatchJobQueueComputeEnvironmentOrderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchJobQueueComputeEnvironmentOrderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `compute_environment` after provisioning.\n"]
    pub fn compute_environment(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.compute_environment", self.base))
    }

    #[doc= "Get a reference to the value of field `order` after provisioning.\n"]
    pub fn order(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.order", self.base))
    }
}
