use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcrRegistryScanningConfigurationData {
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
    scan_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<EcrRegistryScanningConfigurationRuleEl>>,
    dynamic: EcrRegistryScanningConfigurationDynamic,
}

struct EcrRegistryScanningConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EcrRegistryScanningConfigurationData>,
}

#[derive(Clone)]
pub struct EcrRegistryScanningConfiguration(std::rc::Rc<EcrRegistryScanningConfiguration_>);

impl EcrRegistryScanningConfiguration {
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

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcrRegistryScanningConfigurationRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = std::option::Option::Some(d);
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

    #[doc = "Get a reference to the value of field `scan_type` after provisioning.\n"]
    pub fn scan_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_type", self.extract_ref()))
    }
}

impl terrars::Referable for EcrRegistryScanningConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EcrRegistryScanningConfiguration { }

impl terrars::ToListMappable for EcrRegistryScanningConfiguration {
    type O = terrars::ListRef<EcrRegistryScanningConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EcrRegistryScanningConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ecr_registry_scanning_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcrRegistryScanningConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub scan_type: terrars::PrimField<std::string::String>,
}

impl BuildEcrRegistryScanningConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> EcrRegistryScanningConfiguration {
        let out = EcrRegistryScanningConfiguration(std::rc::Rc::new(EcrRegistryScanningConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EcrRegistryScanningConfigurationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                scan_type: self.scan_type,
                rule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcrRegistryScanningConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrRegistryScanningConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EcrRegistryScanningConfigurationRef {
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

    #[doc = "Get a reference to the value of field `scan_type` after provisioning.\n"]
    pub fn scan_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EcrRegistryScanningConfigurationRuleElRepositoryFilterEl {
    filter: terrars::PrimField<std::string::String>,
    filter_type: terrars::PrimField<std::string::String>,
}

impl EcrRegistryScanningConfigurationRuleElRepositoryFilterEl { }

impl terrars::ToListMappable for EcrRegistryScanningConfigurationRuleElRepositoryFilterEl {
    type O = terrars::BlockAssignable<EcrRegistryScanningConfigurationRuleElRepositoryFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrRegistryScanningConfigurationRuleElRepositoryFilterEl {
    #[doc = ""]
    pub filter: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub filter_type: terrars::PrimField<std::string::String>,
}

impl BuildEcrRegistryScanningConfigurationRuleElRepositoryFilterEl {
    pub fn build(self) -> EcrRegistryScanningConfigurationRuleElRepositoryFilterEl {
        EcrRegistryScanningConfigurationRuleElRepositoryFilterEl {
            filter: self.filter,
            filter_type: self.filter_type,
        }
    }
}

pub struct EcrRegistryScanningConfigurationRuleElRepositoryFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrRegistryScanningConfigurationRuleElRepositoryFilterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcrRegistryScanningConfigurationRuleElRepositoryFilterElRef {
        EcrRegistryScanningConfigurationRuleElRepositoryFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrRegistryScanningConfigurationRuleElRepositoryFilterElRef {
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
struct EcrRegistryScanningConfigurationRuleElDynamic {
    repository_filter: std
    ::option
    ::Option<terrars::DynamicBlock<EcrRegistryScanningConfigurationRuleElRepositoryFilterEl>>,
}

#[derive(Serialize)]
pub struct EcrRegistryScanningConfigurationRuleEl {
    scan_frequency: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_filter: std::option::Option<std::vec::Vec<EcrRegistryScanningConfigurationRuleElRepositoryFilterEl>>,
    dynamic: EcrRegistryScanningConfigurationRuleElDynamic,
}

impl EcrRegistryScanningConfigurationRuleEl {
    #[doc = "Set the field `repository_filter`.\n"]
    pub fn set_repository_filter(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EcrRegistryScanningConfigurationRuleElRepositoryFilterEl>>,
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

impl terrars::ToListMappable for EcrRegistryScanningConfigurationRuleEl {
    type O = terrars::BlockAssignable<EcrRegistryScanningConfigurationRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrRegistryScanningConfigurationRuleEl {
    #[doc = ""]
    pub scan_frequency: terrars::PrimField<std::string::String>,
}

impl BuildEcrRegistryScanningConfigurationRuleEl {
    pub fn build(self) -> EcrRegistryScanningConfigurationRuleEl {
        EcrRegistryScanningConfigurationRuleEl {
            scan_frequency: self.scan_frequency,
            repository_filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EcrRegistryScanningConfigurationRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrRegistryScanningConfigurationRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcrRegistryScanningConfigurationRuleElRef {
        EcrRegistryScanningConfigurationRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrRegistryScanningConfigurationRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scan_frequency` after provisioning.\n"]
    pub fn scan_frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_frequency", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcrRegistryScanningConfigurationDynamic {
    rule: std::option::Option<terrars::DynamicBlock<EcrRegistryScanningConfigurationRuleEl>>,
}
