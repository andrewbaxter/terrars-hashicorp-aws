use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppmeshVirtualRouterData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    mesh_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mesh_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spec: std::option::Option<std::vec::Vec<AppmeshVirtualRouterSpecEl>>,
    dynamic: AppmeshVirtualRouterDynamic,
}

struct AppmeshVirtualRouter_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppmeshVirtualRouterData>,
}

#[derive(Clone)]
pub struct AppmeshVirtualRouter(std::rc::Rc<AppmeshVirtualRouter_>);

impl AppmeshVirtualRouter {
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

    #[doc = "Set the field `mesh_owner`.\n"]
    pub fn set_mesh_owner(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().mesh_owner = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `spec`.\n"]
    pub fn set_spec(self, v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualRouterSpecEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().spec = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.spec = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<AppmeshVirtualRouterSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }
}

impl terrars::Referable for AppmeshVirtualRouter {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppmeshVirtualRouter { }

impl terrars::ToListMappable for AppmeshVirtualRouter {
    type O = terrars::ListRef<AppmeshVirtualRouterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppmeshVirtualRouter_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appmesh_virtual_router".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppmeshVirtualRouter {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub mesh_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualRouter {
    pub fn build(self, stack: &mut terrars::Stack) -> AppmeshVirtualRouter {
        let out = AppmeshVirtualRouter(std::rc::Rc::new(AppmeshVirtualRouter_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppmeshVirtualRouterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                mesh_name: self.mesh_name,
                mesh_owner: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                spec: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppmeshVirtualRouterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualRouterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppmeshVirtualRouterRef {
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

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<AppmeshVirtualRouterSpecElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AppmeshVirtualRouterSpecElListenerElPortMappingEl {
    port: terrars::PrimField<f64>,
    protocol: terrars::PrimField<std::string::String>,
}

impl AppmeshVirtualRouterSpecElListenerElPortMappingEl { }

impl terrars::ToListMappable for AppmeshVirtualRouterSpecElListenerElPortMappingEl {
    type O = terrars::BlockAssignable<AppmeshVirtualRouterSpecElListenerElPortMappingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualRouterSpecElListenerElPortMappingEl {
    #[doc = ""]
    pub port: terrars::PrimField<f64>,
    #[doc = ""]
    pub protocol: terrars::PrimField<std::string::String>,
}

impl BuildAppmeshVirtualRouterSpecElListenerElPortMappingEl {
    pub fn build(self) -> AppmeshVirtualRouterSpecElListenerElPortMappingEl {
        AppmeshVirtualRouterSpecElListenerElPortMappingEl {
            port: self.port,
            protocol: self.protocol,
        }
    }
}

pub struct AppmeshVirtualRouterSpecElListenerElPortMappingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualRouterSpecElListenerElPortMappingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppmeshVirtualRouterSpecElListenerElPortMappingElRef {
        AppmeshVirtualRouterSpecElListenerElPortMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualRouterSpecElListenerElPortMappingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualRouterSpecElListenerElDynamic {
    port_mapping: std::option::Option<terrars::DynamicBlock<AppmeshVirtualRouterSpecElListenerElPortMappingEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualRouterSpecElListenerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_mapping: std::option::Option<std::vec::Vec<AppmeshVirtualRouterSpecElListenerElPortMappingEl>>,
    dynamic: AppmeshVirtualRouterSpecElListenerElDynamic,
}

impl AppmeshVirtualRouterSpecElListenerEl {
    #[doc = "Set the field `port_mapping`.\n"]
    pub fn set_port_mapping(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualRouterSpecElListenerElPortMappingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.port_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.port_mapping = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualRouterSpecElListenerEl {
    type O = terrars::BlockAssignable<AppmeshVirtualRouterSpecElListenerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualRouterSpecElListenerEl {}

impl BuildAppmeshVirtualRouterSpecElListenerEl {
    pub fn build(self) -> AppmeshVirtualRouterSpecElListenerEl {
        AppmeshVirtualRouterSpecElListenerEl {
            port_mapping: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualRouterSpecElListenerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualRouterSpecElListenerElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualRouterSpecElListenerElRef {
        AppmeshVirtualRouterSpecElListenerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualRouterSpecElListenerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port_mapping` after provisioning.\n"]
    pub fn port_mapping(&self) -> terrars::ListRef<AppmeshVirtualRouterSpecElListenerElPortMappingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_mapping", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualRouterSpecElDynamic {
    listener: std::option::Option<terrars::DynamicBlock<AppmeshVirtualRouterSpecElListenerEl>>,
}

#[derive(Serialize)]
pub struct AppmeshVirtualRouterSpecEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    listener: std::option::Option<std::vec::Vec<AppmeshVirtualRouterSpecElListenerEl>>,
    dynamic: AppmeshVirtualRouterSpecElDynamic,
}

impl AppmeshVirtualRouterSpecEl {
    #[doc = "Set the field `listener`.\n"]
    pub fn set_listener(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppmeshVirtualRouterSpecElListenerEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.listener = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.listener = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AppmeshVirtualRouterSpecEl {
    type O = terrars::BlockAssignable<AppmeshVirtualRouterSpecEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppmeshVirtualRouterSpecEl {}

impl BuildAppmeshVirtualRouterSpecEl {
    pub fn build(self) -> AppmeshVirtualRouterSpecEl {
        AppmeshVirtualRouterSpecEl {
            listener: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AppmeshVirtualRouterSpecElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppmeshVirtualRouterSpecElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AppmeshVirtualRouterSpecElRef {
        AppmeshVirtualRouterSpecElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppmeshVirtualRouterSpecElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `listener` after provisioning.\n"]
    pub fn listener(&self) -> terrars::ListRef<AppmeshVirtualRouterSpecElListenerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.listener", self.base))
    }
}

#[derive(Serialize, Default)]
struct AppmeshVirtualRouterDynamic {
    spec: std::option::Option<terrars::DynamicBlock<AppmeshVirtualRouterSpecEl>>,
}
