use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EmrcontainersVirtualClusterData {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_provider: std::option::Option<std::vec::Vec<EmrcontainersVirtualClusterContainerProviderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EmrcontainersVirtualClusterTimeoutsEl>,
    dynamic: EmrcontainersVirtualClusterDynamic,
}

struct EmrcontainersVirtualCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EmrcontainersVirtualClusterData>,
}

#[derive(Clone)]
pub struct EmrcontainersVirtualCluster(std::rc::Rc<EmrcontainersVirtualCluster_>);

impl EmrcontainersVirtualCluster {
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

    #[doc = "Set the field `container_provider`.\n"]
    pub fn set_container_provider(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrcontainersVirtualClusterContainerProviderEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().container_provider = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.container_provider = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EmrcontainersVirtualClusterTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `container_provider` after provisioning.\n"]
    pub fn container_provider(&self) -> terrars::ListRef<EmrcontainersVirtualClusterContainerProviderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EmrcontainersVirtualClusterTimeoutsElRef {
        EmrcontainersVirtualClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for EmrcontainersVirtualCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EmrcontainersVirtualCluster { }

impl terrars::ToListMappable for EmrcontainersVirtualCluster {
    type O = terrars::ListRef<EmrcontainersVirtualClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EmrcontainersVirtualCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_emrcontainers_virtual_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmrcontainersVirtualCluster {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildEmrcontainersVirtualCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> EmrcontainersVirtualCluster {
        let out = EmrcontainersVirtualCluster(std::rc::Rc::new(EmrcontainersVirtualCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EmrcontainersVirtualClusterData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                container_provider: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmrcontainersVirtualClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrcontainersVirtualClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EmrcontainersVirtualClusterRef {
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

    #[doc = "Get a reference to the value of field `container_provider` after provisioning.\n"]
    pub fn container_provider(&self) -> terrars::ListRef<EmrcontainersVirtualClusterContainerProviderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EmrcontainersVirtualClusterTimeoutsElRef {
        EmrcontainersVirtualClusterTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    namespace: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {
    #[doc = "Set the field `namespace`.\n"]
    pub fn set_namespace(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.namespace = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {
    type O = terrars::BlockAssignable<EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {}

impl BuildEmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {
    pub fn build(self) -> EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {
        EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl {
            namespace: core::default::Default::default(),
        }
    }
}

pub struct EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoElRef {
        EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrcontainersVirtualClusterContainerProviderElInfoElDynamic {
    eks_info: std
    ::option
    ::Option<terrars::DynamicBlock<EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl>>,
}

#[derive(Serialize)]
pub struct EmrcontainersVirtualClusterContainerProviderElInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eks_info: std::option::Option<std::vec::Vec<EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl>>,
    dynamic: EmrcontainersVirtualClusterContainerProviderElInfoElDynamic,
}

impl EmrcontainersVirtualClusterContainerProviderElInfoEl {
    #[doc = "Set the field `eks_info`.\n"]
    pub fn set_eks_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.eks_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.eks_info = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EmrcontainersVirtualClusterContainerProviderElInfoEl {
    type O = terrars::BlockAssignable<EmrcontainersVirtualClusterContainerProviderElInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersVirtualClusterContainerProviderElInfoEl {}

impl BuildEmrcontainersVirtualClusterContainerProviderElInfoEl {
    pub fn build(self) -> EmrcontainersVirtualClusterContainerProviderElInfoEl {
        EmrcontainersVirtualClusterContainerProviderElInfoEl {
            eks_info: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EmrcontainersVirtualClusterContainerProviderElInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrcontainersVirtualClusterContainerProviderElInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrcontainersVirtualClusterContainerProviderElInfoElRef {
        EmrcontainersVirtualClusterContainerProviderElInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersVirtualClusterContainerProviderElInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eks_info` after provisioning.\n"]
    pub fn eks_info(&self) -> terrars::ListRef<EmrcontainersVirtualClusterContainerProviderElInfoElEksInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.eks_info", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrcontainersVirtualClusterContainerProviderElDynamic {
    info: std::option::Option<terrars::DynamicBlock<EmrcontainersVirtualClusterContainerProviderElInfoEl>>,
}

#[derive(Serialize)]
pub struct EmrcontainersVirtualClusterContainerProviderEl {
    id: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    info: std::option::Option<std::vec::Vec<EmrcontainersVirtualClusterContainerProviderElInfoEl>>,
    dynamic: EmrcontainersVirtualClusterContainerProviderElDynamic,
}

impl EmrcontainersVirtualClusterContainerProviderEl {
    #[doc = "Set the field `info`.\n"]
    pub fn set_info(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrcontainersVirtualClusterContainerProviderElInfoEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.info = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EmrcontainersVirtualClusterContainerProviderEl {
    type O = terrars::BlockAssignable<EmrcontainersVirtualClusterContainerProviderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersVirtualClusterContainerProviderEl {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildEmrcontainersVirtualClusterContainerProviderEl {
    pub fn build(self) -> EmrcontainersVirtualClusterContainerProviderEl {
        EmrcontainersVirtualClusterContainerProviderEl {
            id: self.id,
            type_: self.type_,
            info: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EmrcontainersVirtualClusterContainerProviderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrcontainersVirtualClusterContainerProviderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrcontainersVirtualClusterContainerProviderElRef {
        EmrcontainersVirtualClusterContainerProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersVirtualClusterContainerProviderElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `info` after provisioning.\n"]
    pub fn info(&self) -> terrars::ListRef<EmrcontainersVirtualClusterContainerProviderElInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.info", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrcontainersVirtualClusterTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EmrcontainersVirtualClusterTimeoutsEl {
    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrcontainersVirtualClusterTimeoutsEl {
    type O = terrars::BlockAssignable<EmrcontainersVirtualClusterTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrcontainersVirtualClusterTimeoutsEl {}

impl BuildEmrcontainersVirtualClusterTimeoutsEl {
    pub fn build(self) -> EmrcontainersVirtualClusterTimeoutsEl {
        EmrcontainersVirtualClusterTimeoutsEl { delete: core::default::Default::default() }
    }
}

pub struct EmrcontainersVirtualClusterTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrcontainersVirtualClusterTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrcontainersVirtualClusterTimeoutsElRef {
        EmrcontainersVirtualClusterTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrcontainersVirtualClusterTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrcontainersVirtualClusterDynamic {
    container_provider: std::option::Option<terrars::DynamicBlock<EmrcontainersVirtualClusterContainerProviderEl>>,
}
