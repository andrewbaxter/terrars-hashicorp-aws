use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Route53recoverycontrolconfigClusterData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
}

struct Route53recoverycontrolconfigCluster_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<Route53recoverycontrolconfigClusterData>,
}

#[derive(Clone)]
pub struct Route53recoverycontrolconfigCluster(Rc<Route53recoverycontrolconfigCluster_>);

impl Route53recoverycontrolconfigCluster {
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

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cluster_endpoints` after provisioning.\n"]
    pub fn cluster_endpoints(&self) -> ListRef<Route53recoverycontrolconfigClusterClusterEndpointsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cluster_endpoints", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

impl Resource for Route53recoverycontrolconfigCluster {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for Route53recoverycontrolconfigCluster {
    type O = ListRef<Route53recoverycontrolconfigClusterRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Route53recoverycontrolconfigCluster_ {
    fn extract_resource_type(&self) -> String {
        "aws_route53recoverycontrolconfig_cluster".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRoute53recoverycontrolconfigCluster {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildRoute53recoverycontrolconfigCluster {
    pub fn build(self, stack: &mut Stack) -> Route53recoverycontrolconfigCluster {
        let out = Route53recoverycontrolconfigCluster(Rc::new(Route53recoverycontrolconfigCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(Route53recoverycontrolconfigClusterData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Route53recoverycontrolconfigClusterRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53recoverycontrolconfigClusterRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Route53recoverycontrolconfigClusterRef {
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

    #[doc= "Get a reference to the value of field `cluster_endpoints` after provisioning.\n"]
    pub fn cluster_endpoints(&self) -> ListRef<Route53recoverycontrolconfigClusterClusterEndpointsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cluster_endpoints", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Route53recoverycontrolconfigClusterClusterEndpointsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<PrimField<String>>,
}

impl Route53recoverycontrolconfigClusterClusterEndpointsEl {
    #[doc= "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint = Some(v.into());
        self
    }

    #[doc= "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.region = Some(v.into());
        self
    }
}

impl ToListMappable for Route53recoverycontrolconfigClusterClusterEndpointsEl {
    type O = BlockAssignable<Route53recoverycontrolconfigClusterClusterEndpointsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRoute53recoverycontrolconfigClusterClusterEndpointsEl {}

impl BuildRoute53recoverycontrolconfigClusterClusterEndpointsEl {
    pub fn build(self) -> Route53recoverycontrolconfigClusterClusterEndpointsEl {
        Route53recoverycontrolconfigClusterClusterEndpointsEl {
            endpoint: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

pub struct Route53recoverycontrolconfigClusterClusterEndpointsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for Route53recoverycontrolconfigClusterClusterEndpointsElRef {
    fn new(shared: StackShared, base: String) -> Route53recoverycontrolconfigClusterClusterEndpointsElRef {
        Route53recoverycontrolconfigClusterClusterEndpointsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Route53recoverycontrolconfigClusterClusterEndpointsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }

    #[doc= "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}
