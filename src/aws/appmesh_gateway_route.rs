use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppmeshGatewayRouteData {
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
    mesh_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mesh_owner: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    virtual_gateway_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spec: Option<Vec<AppmeshGatewayRouteSpecEl>>,
    dynamic: AppmeshGatewayRouteDynamic,
}

struct AppmeshGatewayRoute_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<AppmeshGatewayRouteData>,
}

#[derive(Clone)]
pub struct AppmeshGatewayRoute(Rc<AppmeshGatewayRoute_>);

impl AppmeshGatewayRoute {
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

    #[doc= "Set the field `mesh_owner`.\n"]
    pub fn set_mesh_owner(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().mesh_owner = Some(v.into());
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

    #[doc= "Set the field `spec`.\n"]
    pub fn set_spec(self, v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spec = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spec = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mesh_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mesh_owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `virtual_gateway_name` after provisioning.\n"]
    pub fn virtual_gateway_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_gateway_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<AppmeshGatewayRouteSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }
}

impl Resource for AppmeshGatewayRoute {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for AppmeshGatewayRoute {
    type O = ListRef<AppmeshGatewayRouteRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for AppmeshGatewayRoute_ {
    fn extract_resource_type(&self) -> String {
        "aws_appmesh_gateway_route".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppmeshGatewayRoute {
    pub tf_id: String,
    #[doc= ""]
    pub mesh_name: PrimField<String>,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub virtual_gateway_name: PrimField<String>,
}

impl BuildAppmeshGatewayRoute {
    pub fn build(self, stack: &mut Stack) -> AppmeshGatewayRoute {
        let out = AppmeshGatewayRoute(Rc::new(AppmeshGatewayRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(AppmeshGatewayRouteData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                mesh_name: self.mesh_name,
                mesh_owner: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                virtual_gateway_name: self.virtual_gateway_name,
                spec: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppmeshGatewayRouteRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppmeshGatewayRouteRef {
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

    #[doc= "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mesh_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mesh_owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.resource_owner", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `virtual_gateway_name` after provisioning.\n"]
    pub fn virtual_gateway_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_gateway_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> ListRef<AppmeshGatewayRouteSpecElRef> {
        ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl {
    virtual_service_name: PrimField<String>,
}

impl AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl { }

impl ToListMappable for AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl {
    #[doc= ""]
    pub virtual_service_name: PrimField<String>,
}

impl BuildAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl {
        AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl {
            virtual_service_name: self.virtual_service_name,
        }
    }
}

pub struct AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceElRef {
        AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `virtual_service_name` after provisioning.\n"]
    pub fn virtual_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_service_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElDynamic {
    virtual_service: Option<DynamicBlock<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_service: Option<Vec<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl>>,
    dynamic: AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElDynamic,
}

impl AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {
    #[doc= "Set the field `virtual_service`.\n"]
    pub fn set_virtual_service(
        mut self,
        v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.virtual_service = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.virtual_service = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {}

impl BuildAppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {
        AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl {
            virtual_service: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElRef {
        AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `virtual_service` after provisioning.\n"]
    pub fn virtual_service(&self) -> ListRef<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElVirtualServiceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.virtual_service", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElGrpcRouteElActionElDynamic {
    target: Option<DynamicBlock<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElGrpcRouteElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<Vec<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl>>,
    dynamic: AppmeshGatewayRouteSpecElGrpcRouteElActionElDynamic,
}

impl AppmeshGatewayRouteSpecElGrpcRouteElActionEl {
    #[doc= "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshGatewayRouteSpecElGrpcRouteElActionEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteElActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElGrpcRouteElActionEl {}

impl BuildAppmeshGatewayRouteSpecElGrpcRouteElActionEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElGrpcRouteElActionEl {
        AppmeshGatewayRouteSpecElGrpcRouteElActionEl {
            target: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElGrpcRouteElActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElGrpcRouteElActionElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElGrpcRouteElActionElRef {
        AppmeshGatewayRouteSpecElGrpcRouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElGrpcRouteElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<AppmeshGatewayRouteSpecElGrpcRouteElActionElTargetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    service_name: PrimField<String>,
}

impl AppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
    #[doc= "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }
}

impl ToListMappable for AppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
    #[doc= ""]
    pub service_name: PrimField<String>,
}

impl BuildAppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
        AppmeshGatewayRouteSpecElGrpcRouteElMatchEl {
            port: core::default::Default::default(),
            service_name: self.service_name,
        }
    }
}

pub struct AppmeshGatewayRouteSpecElGrpcRouteElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElGrpcRouteElMatchElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElGrpcRouteElMatchElRef {
        AppmeshGatewayRouteSpecElGrpcRouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElGrpcRouteElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc= "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElGrpcRouteElDynamic {
    action: Option<DynamicBlock<AppmeshGatewayRouteSpecElGrpcRouteElActionEl>>,
    match_: Option<DynamicBlock<AppmeshGatewayRouteSpecElGrpcRouteElMatchEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElGrpcRouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<AppmeshGatewayRouteSpecElGrpcRouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<AppmeshGatewayRouteSpecElGrpcRouteElMatchEl>>,
    dynamic: AppmeshGatewayRouteSpecElGrpcRouteElDynamic,
}

impl AppmeshGatewayRouteSpecElGrpcRouteEl {
    #[doc= "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteElActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `match_`.\n"]
    pub fn set_match(mut self, v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteElMatchEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshGatewayRouteSpecElGrpcRouteEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElGrpcRouteEl {}

impl BuildAppmeshGatewayRouteSpecElGrpcRouteEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElGrpcRouteEl {
        AppmeshGatewayRouteSpecElGrpcRouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElGrpcRouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElGrpcRouteElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElGrpcRouteElRef {
        AppmeshGatewayRouteSpecElGrpcRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElGrpcRouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<AppmeshGatewayRouteSpecElGrpcRouteElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc= "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<AppmeshGatewayRouteSpecElGrpcRouteElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl {
    default_target_hostname: PrimField<String>,
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl { }

impl ToListMappable for AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl {
    #[doc= ""]
    pub default_target_hostname: PrimField<String>,
}

impl BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl {
            default_target_hostname: self.default_target_hostname,
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameElRef {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `default_target_hostname` after provisioning.\n"]
    pub fn default_target_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_target_hostname", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {
    #[doc= "Set the field `default_prefix`.\n"]
    pub fn set_default_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {}

impl BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl {
            default_prefix: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixElRef {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `default_prefix` after provisioning.\n"]
    pub fn default_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElDynamic {
    hostname: Option<DynamicBlock<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl>>,
    prefix: Option<DynamicBlock<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<Vec<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<Vec<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElDynamic,
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {
    #[doc= "Set the field `hostname`.\n"]
    pub fn set_hostname(
        mut self,
        v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hostname = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hostname = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `prefix`.\n"]
    pub fn set_prefix(
        mut self,
        v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.prefix = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.prefix = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {}

impl BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl {
            hostname: core::default::Default::default(),
            prefix: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElRef {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> ListRef<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElHostnameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> ListRef<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElPrefixElRef> {
        ListRef::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl {
    virtual_service_name: PrimField<String>,
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl { }

impl ToListMappable for AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl {
    #[doc= ""]
    pub virtual_service_name: PrimField<String>,
}

impl BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl {
            virtual_service_name: self.virtual_service_name,
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceElRef {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `virtual_service_name` after provisioning.\n"]
    pub fn virtual_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_service_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElDynamic {
    virtual_service: Option<DynamicBlock<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_service: Option<Vec<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElDynamic,
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {
    #[doc= "Set the field `virtual_service`.\n"]
    pub fn set_virtual_service(
        mut self,
        v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.virtual_service = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.virtual_service = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {}

impl BuildAppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl {
            virtual_service: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElRef {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `virtual_service` after provisioning.\n"]
    pub fn virtual_service(&self) -> ListRef<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElVirtualServiceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.virtual_service", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttp2RouteElActionElDynamic {
    rewrite: Option<DynamicBlock<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl>>,
    target: Option<DynamicBlock<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rewrite: Option<Vec<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<Vec<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttp2RouteElActionElDynamic,
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionEl {
    #[doc= "Set the field `rewrite`.\n"]
    pub fn set_rewrite(
        mut self,
        v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rewrite = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rewrite = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshGatewayRouteSpecElHttp2RouteElActionEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttp2RouteElActionEl {}

impl BuildAppmeshGatewayRouteSpecElHttp2RouteElActionEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttp2RouteElActionEl {
        AppmeshGatewayRouteSpecElHttp2RouteElActionEl {
            rewrite: core::default::Default::default(),
            target: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttp2RouteElActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElHttp2RouteElActionElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElHttp2RouteElActionElRef {
        AppmeshGatewayRouteSpecElHttp2RouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttp2RouteElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `rewrite` after provisioning.\n"]
    pub fn rewrite(&self) -> ListRef<AppmeshGatewayRouteSpecElHttp2RouteElActionElRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rewrite", self.base))
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<AppmeshGatewayRouteSpecElHttp2RouteElActionElTargetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}

impl AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {
    #[doc= "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }

    #[doc= "Set the field `suffix`.\n"]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
}

impl ToListMappable for AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {}

impl BuildAppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {
        AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl {
            exact: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameElRef {
        AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc= "Get a reference to the value of field `suffix` after provisioning.\n"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttp2RouteElMatchElDynamic {
    hostname: Option<DynamicBlock<AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttp2RouteElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<Vec<AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttp2RouteElMatchElDynamic,
}

impl AppmeshGatewayRouteSpecElHttp2RouteElMatchEl {
    #[doc= "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `hostname`.\n"]
    pub fn set_hostname(
        mut self,
        v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hostname = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hostname = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshGatewayRouteSpecElHttp2RouteElMatchEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttp2RouteElMatchEl {}

impl BuildAppmeshGatewayRouteSpecElHttp2RouteElMatchEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttp2RouteElMatchEl {
        AppmeshGatewayRouteSpecElHttp2RouteElMatchEl {
            port: core::default::Default::default(),
            prefix: core::default::Default::default(),
            hostname: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttp2RouteElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElHttp2RouteElMatchElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElHttp2RouteElMatchElRef {
        AppmeshGatewayRouteSpecElHttp2RouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttp2RouteElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> ListRef<AppmeshGatewayRouteSpecElHttp2RouteElMatchElHostnameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttp2RouteElDynamic {
    action: Option<DynamicBlock<AppmeshGatewayRouteSpecElHttp2RouteElActionEl>>,
    match_: Option<DynamicBlock<AppmeshGatewayRouteSpecElHttp2RouteElMatchEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttp2RouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<AppmeshGatewayRouteSpecElHttp2RouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<AppmeshGatewayRouteSpecElHttp2RouteElMatchEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttp2RouteElDynamic,
}

impl AppmeshGatewayRouteSpecElHttp2RouteEl {
    #[doc= "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `match_`.\n"]
    pub fn set_match(mut self, v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteElMatchEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshGatewayRouteSpecElHttp2RouteEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttp2RouteEl {}

impl BuildAppmeshGatewayRouteSpecElHttp2RouteEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttp2RouteEl {
        AppmeshGatewayRouteSpecElHttp2RouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttp2RouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElHttp2RouteElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElHttp2RouteElRef {
        AppmeshGatewayRouteSpecElHttp2RouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttp2RouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<AppmeshGatewayRouteSpecElHttp2RouteElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc= "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<AppmeshGatewayRouteSpecElHttp2RouteElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl {
    default_target_hostname: PrimField<String>,
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl { }

impl ToListMappable for AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl {
    #[doc= ""]
    pub default_target_hostname: PrimField<String>,
}

impl BuildAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl {
        AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl {
            default_target_hostname: self.default_target_hostname,
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameElRef {
        AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `default_target_hostname` after provisioning.\n"]
    pub fn default_target_hostname(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_target_hostname", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<PrimField<String>>,
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {
    #[doc= "Set the field `default_prefix`.\n"]
    pub fn set_default_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.default_prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.value = Some(v.into());
        self
    }
}

impl ToListMappable for AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {}

impl BuildAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {
        AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl {
            default_prefix: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixElRef {
        AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `default_prefix` after provisioning.\n"]
    pub fn default_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElDynamic {
    hostname: Option<DynamicBlock<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl>>,
    prefix: Option<DynamicBlock<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<Vec<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<Vec<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElDynamic,
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {
    #[doc= "Set the field `hostname`.\n"]
    pub fn set_hostname(
        mut self,
        v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hostname = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hostname = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `prefix`.\n"]
    pub fn set_prefix(
        mut self,
        v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.prefix = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.prefix = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {}

impl BuildAppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {
        AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl {
            hostname: core::default::Default::default(),
            prefix: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElRef {
        AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> ListRef<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElHostnameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> ListRef<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElPrefixElRef> {
        ListRef::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl {
    virtual_service_name: PrimField<String>,
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl { }

impl ToListMappable for AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl {
    #[doc= ""]
    pub virtual_service_name: PrimField<String>,
}

impl BuildAppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl {
        AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl {
            virtual_service_name: self.virtual_service_name,
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceElRef {
        AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `virtual_service_name` after provisioning.\n"]
    pub fn virtual_service_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.virtual_service_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElDynamic {
    virtual_service: Option<DynamicBlock<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    virtual_service: Option<Vec<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElDynamic,
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {
    #[doc= "Set the field `virtual_service`.\n"]
    pub fn set_virtual_service(
        mut self,
        v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.virtual_service = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.virtual_service = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {}

impl BuildAppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {
        AppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl {
            virtual_service: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElRef {
        AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `virtual_service` after provisioning.\n"]
    pub fn virtual_service(&self) -> ListRef<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElVirtualServiceElRef> {
        ListRef::new(self.shared().clone(), format!("{}.virtual_service", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttpRouteElActionElDynamic {
    rewrite: Option<DynamicBlock<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl>>,
    target: Option<DynamicBlock<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttpRouteElActionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    rewrite: Option<Vec<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<Vec<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttpRouteElActionElDynamic,
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionEl {
    #[doc= "Set the field `rewrite`.\n"]
    pub fn set_rewrite(
        mut self,
        v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.rewrite = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.rewrite = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.target = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.target = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshGatewayRouteSpecElHttpRouteElActionEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttpRouteElActionEl {}

impl BuildAppmeshGatewayRouteSpecElHttpRouteElActionEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttpRouteElActionEl {
        AppmeshGatewayRouteSpecElHttpRouteElActionEl {
            rewrite: core::default::Default::default(),
            target: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttpRouteElActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElHttpRouteElActionElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElHttpRouteElActionElRef {
        AppmeshGatewayRouteSpecElHttpRouteElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttpRouteElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `rewrite` after provisioning.\n"]
    pub fn rewrite(&self) -> ListRef<AppmeshGatewayRouteSpecElHttpRouteElActionElRewriteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.rewrite", self.base))
    }

    #[doc= "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> ListRef<AppmeshGatewayRouteSpecElHttpRouteElActionElTargetElRef> {
        ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exact: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suffix: Option<PrimField<String>>,
}

impl AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {
    #[doc= "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.exact = Some(v.into());
        self
    }

    #[doc= "Set the field `suffix`.\n"]
    pub fn set_suffix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.suffix = Some(v.into());
        self
    }
}

impl ToListMappable for AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {}

impl BuildAppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {
        AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl {
            exact: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameElRef {
        AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc= "Get a reference to the value of field `suffix` after provisioning.\n"]
    pub fn suffix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttpRouteElMatchElDynamic {
    hostname: Option<DynamicBlock<AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttpRouteElMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<Vec<AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttpRouteElMatchElDynamic,
}

impl AppmeshGatewayRouteSpecElHttpRouteElMatchEl {
    #[doc= "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.port = Some(v.into());
        self
    }

    #[doc= "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.prefix = Some(v.into());
        self
    }

    #[doc= "Set the field `hostname`.\n"]
    pub fn set_hostname(
        mut self,
        v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.hostname = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.hostname = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshGatewayRouteSpecElHttpRouteElMatchEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttpRouteElMatchEl {}

impl BuildAppmeshGatewayRouteSpecElHttpRouteElMatchEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttpRouteElMatchEl {
        AppmeshGatewayRouteSpecElHttpRouteElMatchEl {
            port: core::default::Default::default(),
            prefix: core::default::Default::default(),
            hostname: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttpRouteElMatchElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElHttpRouteElMatchElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElHttpRouteElMatchElRef {
        AppmeshGatewayRouteSpecElHttpRouteElMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttpRouteElMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc= "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc= "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> ListRef<AppmeshGatewayRouteSpecElHttpRouteElMatchElHostnameElRef> {
        ListRef::new(self.shared().clone(), format!("{}.hostname", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElHttpRouteElDynamic {
    action: Option<DynamicBlock<AppmeshGatewayRouteSpecElHttpRouteElActionEl>>,
    match_: Option<DynamicBlock<AppmeshGatewayRouteSpecElHttpRouteElMatchEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecElHttpRouteEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<AppmeshGatewayRouteSpecElHttpRouteElActionEl>>,
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    match_: Option<Vec<AppmeshGatewayRouteSpecElHttpRouteElMatchEl>>,
    dynamic: AppmeshGatewayRouteSpecElHttpRouteElDynamic,
}

impl AppmeshGatewayRouteSpecElHttpRouteEl {
    #[doc= "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `match_`.\n"]
    pub fn set_match(mut self, v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteElMatchEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.match_ = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshGatewayRouteSpecElHttpRouteEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecElHttpRouteEl {}

impl BuildAppmeshGatewayRouteSpecElHttpRouteEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecElHttpRouteEl {
        AppmeshGatewayRouteSpecElHttpRouteEl {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElHttpRouteElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElHttpRouteElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElHttpRouteElRef {
        AppmeshGatewayRouteSpecElHttpRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElHttpRouteElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<AppmeshGatewayRouteSpecElHttpRouteElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc= "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> ListRef<AppmeshGatewayRouteSpecElHttpRouteElMatchElRef> {
        ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteSpecElDynamic {
    grpc_route: Option<DynamicBlock<AppmeshGatewayRouteSpecElGrpcRouteEl>>,
    http2_route: Option<DynamicBlock<AppmeshGatewayRouteSpecElHttp2RouteEl>>,
    http_route: Option<DynamicBlock<AppmeshGatewayRouteSpecElHttpRouteEl>>,
}

#[derive(Serialize)]
pub struct AppmeshGatewayRouteSpecEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    grpc_route: Option<Vec<AppmeshGatewayRouteSpecElGrpcRouteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http2_route: Option<Vec<AppmeshGatewayRouteSpecElHttp2RouteEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    http_route: Option<Vec<AppmeshGatewayRouteSpecElHttpRouteEl>>,
    dynamic: AppmeshGatewayRouteSpecElDynamic,
}

impl AppmeshGatewayRouteSpecEl {
    #[doc= "Set the field `grpc_route`.\n"]
    pub fn set_grpc_route(mut self, v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElGrpcRouteEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.grpc_route = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.grpc_route = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `http2_route`.\n"]
    pub fn set_http2_route(mut self, v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElHttp2RouteEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http2_route = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http2_route = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `http_route`.\n"]
    pub fn set_http_route(mut self, v: impl Into<BlockAssignable<AppmeshGatewayRouteSpecElHttpRouteEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.http_route = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.http_route = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for AppmeshGatewayRouteSpecEl {
    type O = BlockAssignable<AppmeshGatewayRouteSpecEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshGatewayRouteSpecEl {}

impl BuildAppmeshGatewayRouteSpecEl {
    pub fn build(self) -> AppmeshGatewayRouteSpecEl {
        AppmeshGatewayRouteSpecEl {
            grpc_route: core::default::Default::default(),
            http2_route: core::default::Default::default(),
            http_route: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct AppmeshGatewayRouteSpecElRef {
    shared: StackShared,
    base: String,
}

impl Ref for AppmeshGatewayRouteSpecElRef {
    fn new(shared: StackShared, base: String) -> AppmeshGatewayRouteSpecElRef {
        AppmeshGatewayRouteSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshGatewayRouteSpecElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `grpc_route` after provisioning.\n"]
    pub fn grpc_route(&self) -> ListRef<AppmeshGatewayRouteSpecElGrpcRouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.grpc_route", self.base))
    }

    #[doc= "Get a reference to the value of field `http2_route` after provisioning.\n"]
    pub fn http2_route(&self) -> ListRef<AppmeshGatewayRouteSpecElHttp2RouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http2_route", self.base))
    }

    #[doc= "Get a reference to the value of field `http_route` after provisioning.\n"]
    pub fn http_route(&self) -> ListRef<AppmeshGatewayRouteSpecElHttpRouteElRef> {
        ListRef::new(self.shared().clone(), format!("{}.http_route", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshGatewayRouteDynamic {
    spec: Option<DynamicBlock<AppmeshGatewayRouteSpecEl>>,
}
