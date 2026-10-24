use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcsClusterCapacityProvidersData {
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
    capacity_providers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    cluster_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_capacity_provider_strategy: std
    ::option
    ::Option<std::vec::Vec<EcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl>>,
    dynamic: EcsClusterCapacityProvidersDynamic,
}

struct EcsClusterCapacityProviders_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EcsClusterCapacityProvidersData>,
}

#[derive(Clone)]
pub struct EcsClusterCapacityProviders(std::rc::Rc<EcsClusterCapacityProviders_>);

impl EcsClusterCapacityProviders {
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

    #[doc = "Set the field `capacity_providers`.\n"]
    pub fn set_capacity_providers(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().capacity_providers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_capacity_provider_strategy`.\n"]
    pub fn set_default_capacity_provider_strategy(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_capacity_provider_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_capacity_provider_strategy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `capacity_providers` after provisioning.\n"]
    pub fn capacity_providers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.capacity_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl terrars::Referable for EcsClusterCapacityProviders {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EcsClusterCapacityProviders { }

impl terrars::ToListMappable for EcsClusterCapacityProviders {
    type O = terrars::ListRef<EcsClusterCapacityProvidersRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EcsClusterCapacityProviders_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ecs_cluster_capacity_providers".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcsClusterCapacityProviders {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_name: terrars::PrimField<std::string::String>,
}

impl BuildEcsClusterCapacityProviders {
    pub fn build(self, stack: &mut terrars::Stack) -> EcsClusterCapacityProviders {
        let out = EcsClusterCapacityProviders(std::rc::Rc::new(EcsClusterCapacityProviders_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EcsClusterCapacityProvidersData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                capacity_providers: core::default::Default::default(),
                cluster_name: self.cluster_name,
                id: core::default::Default::default(),
                default_capacity_provider_strategy: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcsClusterCapacityProvidersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsClusterCapacityProvidersRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EcsClusterCapacityProvidersRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_providers` after provisioning.\n"]
    pub fn capacity_providers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.capacity_providers", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base: std::option::Option<terrars::PrimField<f64>>,
    capacity_provider: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl {
    #[doc = "Set the field `base`.\n"]
    pub fn set_base(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.base = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl {
    type O = terrars::BlockAssignable<EcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl {
    #[doc = ""]
    pub capacity_provider: terrars::PrimField<std::string::String>,
}

impl BuildEcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl {
    pub fn build(self) -> EcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl {
        EcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl {
            base: core::default::Default::default(),
            capacity_provider: self.capacity_provider,
            weight: core::default::Default::default(),
        }
    }
}

pub struct EcsClusterCapacityProvidersDefaultCapacityProviderStrategyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsClusterCapacityProvidersDefaultCapacityProviderStrategyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsClusterCapacityProvidersDefaultCapacityProviderStrategyElRef {
        EcsClusterCapacityProvidersDefaultCapacityProviderStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsClusterCapacityProvidersDefaultCapacityProviderStrategyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base` after provisioning.\n"]
    pub fn base(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_provider` after provisioning.\n"]
    pub fn capacity_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_provider", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsClusterCapacityProvidersDynamic {
    default_capacity_provider_strategy: std
    ::option
    ::Option<terrars::DynamicBlock<EcsClusterCapacityProvidersDefaultCapacityProviderStrategyEl>>,
}
