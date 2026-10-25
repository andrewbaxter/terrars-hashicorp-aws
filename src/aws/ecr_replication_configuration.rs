#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcrReplicationConfigurationData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replication_configuration: std::option::Option<std::vec::Vec<EcrReplicationConfiguration_ReplicationConfiguration>>,
    dynamic: EcrReplicationConfigurationDynamic,
}

struct EcrReplicationConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EcrReplicationConfigurationData_>,
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `replication_configuration`.\n"]
    pub fn set_replication_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcrReplicationConfiguration_ReplicationConfiguration>>,
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_configuration` after provisioning.\n"]
    pub fn replication_configuration(&self) -> terrars::ListRef<EcrReplicationConfiguration_ReplicationConfigurationRef> {
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
            data: std::cell::RefCell::new(EcrReplicationConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registry_id` after provisioning.\n"]
    pub fn registry_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `replication_configuration` after provisioning.\n"]
    pub fn replication_configuration(&self) -> terrars::ListRef<EcrReplicationConfiguration_ReplicationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replication_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct EcrReplicationConfigurationDynamic {
    replication_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcrReplicationConfiguration_ReplicationConfiguration>>,
}

#[derive(Serialize)]
pub struct EcrReplicationConfiguration_ReplicationConfiguration_Rule_Destination {
    region: terrars::PrimField<std::string::String>,
    registry_id: terrars::PrimField<std::string::String>,
}

impl EcrReplicationConfiguration_ReplicationConfiguration_Rule_Destination { }

impl terrars::ToListMappable for EcrReplicationConfiguration_ReplicationConfiguration_Rule_Destination {
    type O = terrars::BlockAssignable<EcrReplicationConfiguration_ReplicationConfiguration_Rule_Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrReplicationConfiguration_ReplicationConfiguration_Rule_Destination {
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub registry_id: terrars::PrimField<std::string::String>,
}

impl BuildEcrReplicationConfiguration_ReplicationConfiguration_Rule_Destination {
    pub fn build(self) -> EcrReplicationConfiguration_ReplicationConfiguration_Rule_Destination {
        EcrReplicationConfiguration_ReplicationConfiguration_Rule_Destination {
            region: self.region,
            registry_id: self.registry_id,
        }
    }
}

#[allow(dead_code)]
pub struct EcrReplicationConfiguration_ReplicationConfiguration_Rule_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrReplicationConfiguration_ReplicationConfiguration_Rule_DestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcrReplicationConfiguration_ReplicationConfiguration_Rule_DestinationRef {
        EcrReplicationConfiguration_ReplicationConfiguration_Rule_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrReplicationConfiguration_ReplicationConfiguration_Rule_DestinationRef {
    #[allow(dead_code)]
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
pub struct EcrReplicationConfiguration_ReplicationConfiguration_Rule_RepositoryFilter {
    filter: terrars::PrimField<std::string::String>,
    filter_type: terrars::PrimField<std::string::String>,
}

impl EcrReplicationConfiguration_ReplicationConfiguration_Rule_RepositoryFilter { }

impl terrars::ToListMappable for EcrReplicationConfiguration_ReplicationConfiguration_Rule_RepositoryFilter {
    type O = terrars::BlockAssignable<EcrReplicationConfiguration_ReplicationConfiguration_Rule_RepositoryFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrReplicationConfiguration_ReplicationConfiguration_Rule_RepositoryFilter {
    #[doc = ""]
    pub filter: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub filter_type: terrars::PrimField<std::string::String>,
}

impl BuildEcrReplicationConfiguration_ReplicationConfiguration_Rule_RepositoryFilter {
    pub fn build(self) -> EcrReplicationConfiguration_ReplicationConfiguration_Rule_RepositoryFilter {
        EcrReplicationConfiguration_ReplicationConfiguration_Rule_RepositoryFilter {
            filter: self.filter,
            filter_type: self.filter_type,
        }
    }
}

#[allow(dead_code)]
pub struct EcrReplicationConfiguration_ReplicationConfiguration_Rule_RepositoryFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrReplicationConfiguration_ReplicationConfiguration_Rule_RepositoryFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcrReplicationConfiguration_ReplicationConfiguration_Rule_RepositoryFilterRef {
        EcrReplicationConfiguration_ReplicationConfiguration_Rule_RepositoryFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrReplicationConfiguration_ReplicationConfiguration_Rule_RepositoryFilterRef {
    #[allow(dead_code)]
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
struct EcrReplicationConfiguration_ReplicationConfiguration_RuleDynamic {
    destination: std
    ::option
    ::Option<terrars::DynamicBlock<EcrReplicationConfiguration_ReplicationConfiguration_Rule_Destination>>,
    repository_filter: std
    ::option
    ::Option<terrars::DynamicBlock<EcrReplicationConfiguration_ReplicationConfiguration_Rule_RepositoryFilter>>,
}

#[derive(Serialize)]
pub struct EcrReplicationConfiguration_ReplicationConfiguration_Rule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std
    ::option
    ::Option<std::vec::Vec<EcrReplicationConfiguration_ReplicationConfiguration_Rule_Destination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_filter: std
    ::option
    ::Option<std::vec::Vec<EcrReplicationConfiguration_ReplicationConfiguration_Rule_RepositoryFilter>>,
    dynamic: EcrReplicationConfiguration_ReplicationConfiguration_RuleDynamic,
}

impl EcrReplicationConfiguration_ReplicationConfiguration_Rule {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<EcrReplicationConfiguration_ReplicationConfiguration_Rule_Destination>,
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
                        ::BlockAssignable<EcrReplicationConfiguration_ReplicationConfiguration_Rule_RepositoryFilter>,
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

impl terrars::ToListMappable for EcrReplicationConfiguration_ReplicationConfiguration_Rule {
    type O = terrars::BlockAssignable<EcrReplicationConfiguration_ReplicationConfiguration_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrReplicationConfiguration_ReplicationConfiguration_Rule {}

impl BuildEcrReplicationConfiguration_ReplicationConfiguration_Rule {
    pub fn build(self) -> EcrReplicationConfiguration_ReplicationConfiguration_Rule {
        EcrReplicationConfiguration_ReplicationConfiguration_Rule {
            destination: core::default::Default::default(),
            repository_filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcrReplicationConfiguration_ReplicationConfiguration_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrReplicationConfiguration_ReplicationConfiguration_RuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcrReplicationConfiguration_ReplicationConfiguration_RuleRef {
        EcrReplicationConfiguration_ReplicationConfiguration_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrReplicationConfiguration_ReplicationConfiguration_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars::ListRef<EcrReplicationConfiguration_ReplicationConfiguration_Rule_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `repository_filter` after provisioning.\n"]
    pub fn repository_filter(
        &self,
    ) -> terrars::ListRef<EcrReplicationConfiguration_ReplicationConfiguration_Rule_RepositoryFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repository_filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcrReplicationConfiguration_ReplicationConfigurationDynamic {
    rule: std::option::Option<terrars::DynamicBlock<EcrReplicationConfiguration_ReplicationConfiguration_Rule>>,
}

#[derive(Serialize)]
pub struct EcrReplicationConfiguration_ReplicationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<EcrReplicationConfiguration_ReplicationConfiguration_Rule>>,
    dynamic: EcrReplicationConfiguration_ReplicationConfigurationDynamic,
}

impl EcrReplicationConfiguration_ReplicationConfiguration {
    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EcrReplicationConfiguration_ReplicationConfiguration_Rule>>,
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

impl terrars::ToListMappable for EcrReplicationConfiguration_ReplicationConfiguration {
    type O = terrars::BlockAssignable<EcrReplicationConfiguration_ReplicationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcrReplicationConfiguration_ReplicationConfiguration {}

impl BuildEcrReplicationConfiguration_ReplicationConfiguration {
    pub fn build(self) -> EcrReplicationConfiguration_ReplicationConfiguration {
        EcrReplicationConfiguration_ReplicationConfiguration {
            rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcrReplicationConfiguration_ReplicationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcrReplicationConfiguration_ReplicationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcrReplicationConfiguration_ReplicationConfigurationRef {
        EcrReplicationConfiguration_ReplicationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcrReplicationConfiguration_ReplicationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<EcrReplicationConfiguration_ReplicationConfiguration_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}
