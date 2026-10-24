use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcrReplicationConfigurationData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_configuration: std::option::Option<std::vec::Vec<EcrReplicationConfigurationReplicationConfigurationEl>>,
    dynamic: EcrReplicationConfigurationDynamic,
}

struct EcrReplicationConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EcrReplicationConfigurationData>,
}

#[derive(Clone)]
pub struct EcrReplicationConfiguration(std::rc::Rc<EcrReplicationConfiguration_>);

impl EcrReplicationConfiguration {
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

    #[doc = "Set the field `replication_configuration`.\n"]
    pub fn set_replication_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcrReplicationConfigurationReplicationConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().replication_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.replication_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_configuration` after provisioning.\n"]
    pub fn replication_configuration(
        &self,
    ) -> terrars::ListRef<EcrReplicationConfigurationReplicationConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for EcrReplicationConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EcrReplicationConfiguration { }

impl terrars::ToListMappable for EcrReplicationConfiguration {
    type O = terrars::ListRef<EcrReplicationConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EcrReplicationConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ecr_replication_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcrReplicationConfiguration {
    pub tf_id: std::string::String,
}

impl BuildEcrReplicationConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> EcrReplicationConfiguration {
        let out = EcrReplicationConfiguration(std::rc::Rc::new(EcrReplicationConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EcrReplicationConfigurationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                replication_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcrReplicationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrReplicationConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EcrReplicationConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_configuration` after provisioning.\n"]
    pub fn replication_configuration(
        &self,
    ) -> terrars::ListRef<EcrReplicationConfigurationReplicationConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl {
    region: terrars::PrimField<std::string::String>,
    registry_id: terrars::PrimField<std::string::String>,
}

impl EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl { }

impl terrars::ToListMappable for EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl {
    type O = terrars::BlockAssignable<EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl {
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub registry_id: terrars::PrimField<std::string::String>,
}

impl BuildEcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl {
    pub fn build(self) -> EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl {
        EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl {
            region: self.region,
            registry_id: self.registry_id,
        }
    }
}

pub struct EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationElRef {
        EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.base))
    }
}

#[derive(Serialize)]
pub struct EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl {
    filter: terrars::PrimField<std::string::String>,
    filter_type: terrars::PrimField<std::string::String>,
}

impl EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl { }

impl terrars::ToListMappable for EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl {
    type O = terrars::BlockAssignable<EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl {
    #[doc = ""]
    pub filter: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub filter_type: terrars::PrimField<std::string::String>,
}

impl BuildEcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl {
    pub fn build(self) -> EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl {
        EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl {
            filter: self.filter,
            filter_type: self.filter_type,
        }
    }
}

pub struct EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterElRef {
        EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `filter_type` after provisioning.\n"]
    pub fn filter_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filter_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcrReplicationConfigurationReplicationConfigurationElRuleElDynamic {
    destination: std
    ::option
    ::Option<terrars::DynamicBlock<EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl>>,
    repository_filter: std
    ::option
    ::Option<terrars::DynamicBlock<EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl>>,
}

#[derive(Serialize)]
pub struct EcrReplicationConfigurationReplicationConfigurationElRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<std::vec::Vec<EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_filter: std
    ::option
    ::Option<std::vec::Vec<EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl>>,
    dynamic: EcrReplicationConfigurationReplicationConfigurationElRuleElDynamic,
}

impl EcrReplicationConfigurationReplicationConfigurationElRuleEl {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `repository_filter`.\n"]
    pub fn set_repository_filter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.repository_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.repository_filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcrReplicationConfigurationReplicationConfigurationElRuleEl {
    type O = terrars::BlockAssignable<EcrReplicationConfigurationReplicationConfigurationElRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrReplicationConfigurationReplicationConfigurationElRuleEl {}

impl BuildEcrReplicationConfigurationReplicationConfigurationElRuleEl {
    pub fn build(self) -> EcrReplicationConfigurationReplicationConfigurationElRuleEl {
        EcrReplicationConfigurationReplicationConfigurationElRuleEl {
            destination: core::default::Default::default(),
            repository_filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EcrReplicationConfigurationReplicationConfigurationElRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrReplicationConfigurationReplicationConfigurationElRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcrReplicationConfigurationReplicationConfigurationElRuleElRef {
        EcrReplicationConfigurationReplicationConfigurationElRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrReplicationConfigurationReplicationConfigurationElRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars::ListRef<EcrReplicationConfigurationReplicationConfigurationElRuleElDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `repository_filter` after provisioning.\n"]
    pub fn repository_filter(
        &self,
    ) -> terrars::ListRef<EcrReplicationConfigurationReplicationConfigurationElRuleElRepositoryFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repository_filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcrReplicationConfigurationReplicationConfigurationElDynamic {
    rule: std::option::Option<terrars::DynamicBlock<EcrReplicationConfigurationReplicationConfigurationElRuleEl>>,
}

#[derive(Serialize)]
pub struct EcrReplicationConfigurationReplicationConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<EcrReplicationConfigurationReplicationConfigurationElRuleEl>>,
    dynamic: EcrReplicationConfigurationReplicationConfigurationElDynamic,
}

impl EcrReplicationConfigurationReplicationConfigurationEl {
    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EcrReplicationConfigurationReplicationConfigurationElRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcrReplicationConfigurationReplicationConfigurationEl {
    type O = terrars::BlockAssignable<EcrReplicationConfigurationReplicationConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrReplicationConfigurationReplicationConfigurationEl {}

impl BuildEcrReplicationConfigurationReplicationConfigurationEl {
    pub fn build(self) -> EcrReplicationConfigurationReplicationConfigurationEl {
        EcrReplicationConfigurationReplicationConfigurationEl {
            rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EcrReplicationConfigurationReplicationConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrReplicationConfigurationReplicationConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcrReplicationConfigurationReplicationConfigurationElRef {
        EcrReplicationConfigurationReplicationConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrReplicationConfigurationReplicationConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<EcrReplicationConfigurationReplicationConfigurationElRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcrReplicationConfigurationDynamic {
    replication_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcrReplicationConfigurationReplicationConfigurationEl>>,
}
