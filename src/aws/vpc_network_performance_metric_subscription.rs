use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VpcNetworkPerformanceMetricSubscriptionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    destination: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metric: Option<PrimField<String>>,
    source: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    statistic: Option<PrimField<String>>,
}

struct VpcNetworkPerformanceMetricSubscription_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<VpcNetworkPerformanceMetricSubscriptionData>,
}

#[derive(Clone)]
pub struct VpcNetworkPerformanceMetricSubscription(Rc<VpcNetworkPerformanceMetricSubscription_>);

impl VpcNetworkPerformanceMetricSubscription {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Resource) -> Self {
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

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `metric`.\n"]
    pub fn set_metric(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().metric = Some(v.into());
        self
    }

    #[doc= "Set the field `statistic`.\n"]
    pub fn set_statistic(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().statistic = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `statistic` after provisioning.\n"]
    pub fn statistic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.statistic", self.extract_ref()))
    }
}

impl Resource for VpcNetworkPerformanceMetricSubscription {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for VpcNetworkPerformanceMetricSubscription {
    type O = ListRef<VpcNetworkPerformanceMetricSubscriptionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for VpcNetworkPerformanceMetricSubscription_ {
    fn extract_resource_type(&self) -> String {
        "aws_vpc_network_performance_metric_subscription".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpcNetworkPerformanceMetricSubscription {
    pub tf_id: String,
    #[doc= ""]
    pub destination: PrimField<String>,
    #[doc= ""]
    pub source: PrimField<String>,
}

impl BuildVpcNetworkPerformanceMetricSubscription {
    pub fn build(self, stack: &mut Stack) -> VpcNetworkPerformanceMetricSubscription {
        let out = VpcNetworkPerformanceMetricSubscription(Rc::new(VpcNetworkPerformanceMetricSubscription_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(VpcNetworkPerformanceMetricSubscriptionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                destination: self.destination,
                id: core::default::Default::default(),
                metric: core::default::Default::default(),
                source: self.source,
                statistic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpcNetworkPerformanceMetricSubscriptionRef {
    shared: StackShared,
    base: String,
}

impl Ref for VpcNetworkPerformanceMetricSubscriptionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VpcNetworkPerformanceMetricSubscriptionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metric` after provisioning.\n"]
    pub fn metric(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.period", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `statistic` after provisioning.\n"]
    pub fn statistic(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.statistic", self.extract_ref()))
    }
}
