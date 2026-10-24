use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConfigConfigurationAggregatorData {
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
    account_aggregation_source: std
    ::option
    ::Option<std::vec::Vec<ConfigConfigurationAggregatorAccountAggregationSourceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization_aggregation_source: std
    ::option
    ::Option<std::vec::Vec<ConfigConfigurationAggregatorOrganizationAggregationSourceEl>>,
    dynamic: ConfigConfigurationAggregatorDynamic,
}

struct ConfigConfigurationAggregator_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ConfigConfigurationAggregatorData>,
}

#[derive(Clone)]
pub struct ConfigConfigurationAggregator(std::rc::Rc<ConfigConfigurationAggregator_>);

impl ConfigConfigurationAggregator {
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

    #[doc = "Set the field `account_aggregation_source`.\n"]
    pub fn set_account_aggregation_source(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ConfigConfigurationAggregatorAccountAggregationSourceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().account_aggregation_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.account_aggregation_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `organization_aggregation_source`.\n"]
    pub fn set_organization_aggregation_source(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ConfigConfigurationAggregatorOrganizationAggregationSourceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().organization_aggregation_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.organization_aggregation_source = std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `account_aggregation_source` after provisioning.\n"]
    pub fn account_aggregation_source(
        &self,
    ) -> terrars::ListRef<ConfigConfigurationAggregatorAccountAggregationSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.account_aggregation_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organization_aggregation_source` after provisioning.\n"]
    pub fn organization_aggregation_source(
        &self,
    ) -> terrars::ListRef<ConfigConfigurationAggregatorOrganizationAggregationSourceElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.organization_aggregation_source", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ConfigConfigurationAggregator {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ConfigConfigurationAggregator { }

impl terrars::ToListMappable for ConfigConfigurationAggregator {
    type O = terrars::ListRef<ConfigConfigurationAggregatorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ConfigConfigurationAggregator_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_config_configuration_aggregator".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConfigConfigurationAggregator {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildConfigConfigurationAggregator {
    pub fn build(self, stack: &mut terrars::Stack) -> ConfigConfigurationAggregator {
        let out = ConfigConfigurationAggregator(std::rc::Rc::new(ConfigConfigurationAggregator_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ConfigConfigurationAggregatorData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                account_aggregation_source: core::default::Default::default(),
                organization_aggregation_source: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConfigConfigurationAggregatorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConfigConfigurationAggregatorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ConfigConfigurationAggregatorRef {
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

    #[doc = "Get a reference to the value of field `account_aggregation_source` after provisioning.\n"]
    pub fn account_aggregation_source(
        &self,
    ) -> terrars::ListRef<ConfigConfigurationAggregatorAccountAggregationSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.account_aggregation_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organization_aggregation_source` after provisioning.\n"]
    pub fn organization_aggregation_source(
        &self,
    ) -> terrars::ListRef<ConfigConfigurationAggregatorOrganizationAggregationSourceElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.organization_aggregation_source", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct ConfigConfigurationAggregatorAccountAggregationSourceEl {
    account_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_regions: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ConfigConfigurationAggregatorAccountAggregationSourceEl {
    #[doc = "Set the field `all_regions`.\n"]
    pub fn set_all_regions(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all_regions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.regions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ConfigConfigurationAggregatorAccountAggregationSourceEl {
    type O = terrars::BlockAssignable<ConfigConfigurationAggregatorAccountAggregationSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConfigConfigurationAggregatorAccountAggregationSourceEl {
    #[doc = ""]
    pub account_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildConfigConfigurationAggregatorAccountAggregationSourceEl {
    pub fn build(self) -> ConfigConfigurationAggregatorAccountAggregationSourceEl {
        ConfigConfigurationAggregatorAccountAggregationSourceEl {
            account_ids: self.account_ids,
            all_regions: core::default::Default::default(),
            regions: core::default::Default::default(),
        }
    }
}

pub struct ConfigConfigurationAggregatorAccountAggregationSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConfigConfigurationAggregatorAccountAggregationSourceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConfigConfigurationAggregatorAccountAggregationSourceElRef {
        ConfigConfigurationAggregatorAccountAggregationSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConfigConfigurationAggregatorAccountAggregationSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_ids` after provisioning.\n"]
    pub fn account_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.account_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `all_regions` after provisioning.\n"]
    pub fn all_regions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all_regions", self.base))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }
}

#[derive(Serialize)]
pub struct ConfigConfigurationAggregatorOrganizationAggregationSourceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_regions: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl ConfigConfigurationAggregatorOrganizationAggregationSourceEl {
    #[doc = "Set the field `all_regions`.\n"]
    pub fn set_all_regions(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.all_regions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regions`.\n"]
    pub fn set_regions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.regions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ConfigConfigurationAggregatorOrganizationAggregationSourceEl {
    type O = terrars::BlockAssignable<ConfigConfigurationAggregatorOrganizationAggregationSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConfigConfigurationAggregatorOrganizationAggregationSourceEl {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildConfigConfigurationAggregatorOrganizationAggregationSourceEl {
    pub fn build(self) -> ConfigConfigurationAggregatorOrganizationAggregationSourceEl {
        ConfigConfigurationAggregatorOrganizationAggregationSourceEl {
            all_regions: core::default::Default::default(),
            regions: core::default::Default::default(),
            role_arn: self.role_arn,
        }
    }
}

pub struct ConfigConfigurationAggregatorOrganizationAggregationSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConfigConfigurationAggregatorOrganizationAggregationSourceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConfigConfigurationAggregatorOrganizationAggregationSourceElRef {
        ConfigConfigurationAggregatorOrganizationAggregationSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConfigConfigurationAggregatorOrganizationAggregationSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all_regions` after provisioning.\n"]
    pub fn all_regions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.all_regions", self.base))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConfigConfigurationAggregatorDynamic {
    account_aggregation_source: std
    ::option
    ::Option<terrars::DynamicBlock<ConfigConfigurationAggregatorAccountAggregationSourceEl>>,
    organization_aggregation_source: std
    ::option
    ::Option<terrars::DynamicBlock<ConfigConfigurationAggregatorOrganizationAggregationSourceEl>>,
}
