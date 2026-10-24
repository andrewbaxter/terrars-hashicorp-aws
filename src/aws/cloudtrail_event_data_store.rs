use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudtrailEventDataStoreData {
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
    multi_region_enabled: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    termination_protection_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_event_selector: std::option::Option<std::vec::Vec<CloudtrailEventDataStoreAdvancedEventSelectorEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<CloudtrailEventDataStoreTimeoutsEl>,
    dynamic: CloudtrailEventDataStoreDynamic,
}

struct CloudtrailEventDataStore_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudtrailEventDataStoreData>,
}

#[derive(Clone)]
pub struct CloudtrailEventDataStore(std::rc::Rc<CloudtrailEventDataStore_>);

impl CloudtrailEventDataStore {
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

    #[doc = "Set the field `multi_region_enabled`.\n"]
    pub fn set_multi_region_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().multi_region_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `organization_enabled`.\n"]
    pub fn set_organization_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().organization_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retention_period`.\n"]
    pub fn set_retention_period(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retention_period = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `termination_protection_enabled`.\n"]
    pub fn set_termination_protection_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().termination_protection_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `advanced_event_selector`.\n"]
    pub fn set_advanced_event_selector(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudtrailEventDataStoreAdvancedEventSelectorEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().advanced_event_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.advanced_event_selector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<CloudtrailEventDataStoreTimeoutsEl>) -> Self {
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

    #[doc = "Get a reference to the value of field `multi_region_enabled` after provisioning.\n"]
    pub fn multi_region_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multi_region_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organization_enabled` after provisioning.\n"]
    pub fn organization_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_period` after provisioning.\n"]
    pub fn retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `termination_protection_enabled` after provisioning.\n"]
    pub fn termination_protection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.termination_protection_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `advanced_event_selector` after provisioning.\n"]
    pub fn advanced_event_selector(&self) -> terrars::ListRef<CloudtrailEventDataStoreAdvancedEventSelectorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_event_selector", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudtrailEventDataStoreTimeoutsElRef {
        CloudtrailEventDataStoreTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for CloudtrailEventDataStore {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudtrailEventDataStore { }

impl terrars::ToListMappable for CloudtrailEventDataStore {
    type O = terrars::ListRef<CloudtrailEventDataStoreRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudtrailEventDataStore_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudtrail_event_data_store".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudtrailEventDataStore {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudtrailEventDataStore {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudtrailEventDataStore {
        let out = CloudtrailEventDataStore(std::rc::Rc::new(CloudtrailEventDataStore_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudtrailEventDataStoreData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                multi_region_enabled: core::default::Default::default(),
                name: self.name,
                organization_enabled: core::default::Default::default(),
                retention_period: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                termination_protection_enabled: core::default::Default::default(),
                advanced_event_selector: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudtrailEventDataStoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudtrailEventDataStoreRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudtrailEventDataStoreRef {
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

    #[doc = "Get a reference to the value of field `multi_region_enabled` after provisioning.\n"]
    pub fn multi_region_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multi_region_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organization_enabled` after provisioning.\n"]
    pub fn organization_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_period` after provisioning.\n"]
    pub fn retention_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `termination_protection_enabled` after provisioning.\n"]
    pub fn termination_protection_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.termination_protection_enabled", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `advanced_event_selector` after provisioning.\n"]
    pub fn advanced_event_selector(&self) -> terrars::ListRef<CloudtrailEventDataStoreAdvancedEventSelectorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_event_selector", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> CloudtrailEventDataStoreTimeoutsElRef {
        CloudtrailEventDataStoreTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ends_with: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    equals: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_ends_with: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_equals: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_starts_with: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    starts_with: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl {
    #[doc = "Set the field `ends_with`.\n"]
    pub fn set_ends_with(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ends_with = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `equals`.\n"]
    pub fn set_equals(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.equals = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_ends_with`.\n"]
    pub fn set_not_ends_with(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.not_ends_with = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_equals`.\n"]
    pub fn set_not_equals(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.not_equals = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_starts_with`.\n"]
    pub fn set_not_starts_with(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.not_starts_with = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `starts_with`.\n"]
    pub fn set_starts_with(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.starts_with = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl {
    type O = terrars::BlockAssignable<CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl {}

impl BuildCloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl {
    pub fn build(self) -> CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl {
        CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl {
            ends_with: core::default::Default::default(),
            equals: core::default::Default::default(),
            field: core::default::Default::default(),
            not_ends_with: core::default::Default::default(),
            not_equals: core::default::Default::default(),
            not_starts_with: core::default::Default::default(),
            starts_with: core::default::Default::default(),
        }
    }
}

pub struct CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorElRef {
        CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ends_with` after provisioning.\n"]
    pub fn ends_with(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ends_with", self.base))
    }

    #[doc = "Get a reference to the value of field `equals` after provisioning.\n"]
    pub fn equals(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.equals", self.base))
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `not_ends_with` after provisioning.\n"]
    pub fn not_ends_with(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_ends_with", self.base))
    }

    #[doc = "Get a reference to the value of field `not_equals` after provisioning.\n"]
    pub fn not_equals(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_equals", self.base))
    }

    #[doc = "Get a reference to the value of field `not_starts_with` after provisioning.\n"]
    pub fn not_starts_with(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_starts_with", self.base))
    }

    #[doc = "Get a reference to the value of field `starts_with` after provisioning.\n"]
    pub fn starts_with(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.starts_with", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudtrailEventDataStoreAdvancedEventSelectorElDynamic {
    field_selector: std
    ::option
    ::Option<terrars::DynamicBlock<CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl>>,
}

#[derive(Serialize)]
pub struct CloudtrailEventDataStoreAdvancedEventSelectorEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_selector: std::option::Option<std::vec::Vec<CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl>>,
    dynamic: CloudtrailEventDataStoreAdvancedEventSelectorElDynamic,
}

impl CloudtrailEventDataStoreAdvancedEventSelectorEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field_selector`.\n"]
    pub fn set_field_selector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudtrailEventDataStoreAdvancedEventSelectorElFieldSelectorEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_selector = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudtrailEventDataStoreAdvancedEventSelectorEl {
    type O = terrars::BlockAssignable<CloudtrailEventDataStoreAdvancedEventSelectorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudtrailEventDataStoreAdvancedEventSelectorEl {}

impl BuildCloudtrailEventDataStoreAdvancedEventSelectorEl {
    pub fn build(self) -> CloudtrailEventDataStoreAdvancedEventSelectorEl {
        CloudtrailEventDataStoreAdvancedEventSelectorEl {
            name: core::default::Default::default(),
            field_selector: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CloudtrailEventDataStoreAdvancedEventSelectorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudtrailEventDataStoreAdvancedEventSelectorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudtrailEventDataStoreAdvancedEventSelectorElRef {
        CloudtrailEventDataStoreAdvancedEventSelectorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudtrailEventDataStoreAdvancedEventSelectorElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudtrailEventDataStoreTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CloudtrailEventDataStoreTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudtrailEventDataStoreTimeoutsEl {
    type O = terrars::BlockAssignable<CloudtrailEventDataStoreTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudtrailEventDataStoreTimeoutsEl {}

impl BuildCloudtrailEventDataStoreTimeoutsEl {
    pub fn build(self) -> CloudtrailEventDataStoreTimeoutsEl {
        CloudtrailEventDataStoreTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct CloudtrailEventDataStoreTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudtrailEventDataStoreTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudtrailEventDataStoreTimeoutsElRef {
        CloudtrailEventDataStoreTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudtrailEventDataStoreTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudtrailEventDataStoreDynamic {
    advanced_event_selector: std
    ::option
    ::Option<terrars::DynamicBlock<CloudtrailEventDataStoreAdvancedEventSelectorEl>>,
}
