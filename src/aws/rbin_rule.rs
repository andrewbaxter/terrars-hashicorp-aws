#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RbinRuleData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_resource_tags: std::option::Option<std::vec::Vec<RbinRule_ExcludeResourceTags>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lock_configuration: std::option::Option<std::vec::Vec<RbinRule_LockConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_tags: std::option::Option<std::vec::Vec<RbinRule_ResourceTags>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_period: std::option::Option<std::vec::Vec<RbinRule_RetentionPeriod>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<RbinRule_Timeouts>,
    dynamic: RbinRuleDynamic,
}

struct RbinRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RbinRuleData_>,
}

#[derive(Clone)]
pub struct RbinRule(std::rc::Rc<RbinRule_>);

impl RbinRule {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `exclude_resource_tags`.\n"]
    pub fn set_exclude_resource_tags(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RbinRule_ExcludeResourceTags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().exclude_resource_tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.exclude_resource_tags = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lock_configuration`.\n"]
    pub fn set_lock_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RbinRule_LockConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lock_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lock_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_tags`.\n"]
    pub fn set_resource_tags(self, v: impl std::convert::Into<terrars::BlockAssignable<RbinRule_ResourceTags>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_tags = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retention_period`.\n"]
    pub fn set_retention_period(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RbinRule_RetentionPeriod>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().retention_period = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.retention_period = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<RbinRule_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lock_end_time` after provisioning.\n"]
    pub fn lock_end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lock_end_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lock_state` after provisioning.\n"]
    pub fn lock_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lock_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lock_configuration` after provisioning.\n"]
    pub fn lock_configuration(&self) -> terrars::ListRef<RbinRule_LockConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lock_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_period` after provisioning.\n"]
    pub fn retention_period(&self) -> terrars::ListRef<RbinRule_RetentionPeriodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RbinRule_TimeoutsRef {
        RbinRule_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for RbinRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for RbinRule { }

impl terrars::ToListMappable for RbinRule {
    type O = terrars::ListRef<RbinRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for RbinRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_rbin_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRbinRule {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub resource_type: terrars::PrimField<std::string::String>,
}

impl BuildRbinRule {
    pub fn build(self, stack: &mut terrars::Stack) -> RbinRule {
        let out = RbinRule(std::rc::Rc::new(RbinRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(RbinRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                region: core::default::Default::default(),
                resource_type: self.resource_type,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                exclude_resource_tags: core::default::Default::default(),
                lock_configuration: core::default::Default::default(),
                resource_tags: core::default::Default::default(),
                retention_period: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RbinRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RbinRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RbinRuleRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lock_end_time` after provisioning.\n"]
    pub fn lock_end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lock_end_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lock_state` after provisioning.\n"]
    pub fn lock_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lock_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lock_configuration` after provisioning.\n"]
    pub fn lock_configuration(&self) -> terrars::ListRef<RbinRule_LockConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lock_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retention_period` after provisioning.\n"]
    pub fn retention_period(&self) -> terrars::ListRef<RbinRule_RetentionPeriodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_period", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> RbinRule_TimeoutsRef {
        RbinRule_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct RbinRuleDynamic {
    exclude_resource_tags: std::option::Option<terrars::DynamicBlock<RbinRule_ExcludeResourceTags>>,
    lock_configuration: std::option::Option<terrars::DynamicBlock<RbinRule_LockConfiguration>>,
    resource_tags: std::option::Option<terrars::DynamicBlock<RbinRule_ResourceTags>>,
    retention_period: std::option::Option<terrars::DynamicBlock<RbinRule_RetentionPeriod>>,
}

#[derive(Serialize)]
pub struct RbinRule_ExcludeResourceTags {
    resource_tag_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_tag_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RbinRule_ExcludeResourceTags {
    #[doc = "Set the field `resource_tag_value`.\n"]
    pub fn set_resource_tag_value(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.resource_tag_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RbinRule_ExcludeResourceTags {
    type O = terrars::BlockAssignable<RbinRule_ExcludeResourceTags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRbinRule_ExcludeResourceTags {
    #[doc = ""]
    pub resource_tag_key: terrars::PrimField<std::string::String>,
}

impl BuildRbinRule_ExcludeResourceTags {
    pub fn build(self) -> RbinRule_ExcludeResourceTags {
        RbinRule_ExcludeResourceTags {
            resource_tag_key: self.resource_tag_key,
            resource_tag_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RbinRule_ExcludeResourceTagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RbinRule_ExcludeResourceTagsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RbinRule_ExcludeResourceTagsRef {
        RbinRule_ExcludeResourceTagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RbinRule_ExcludeResourceTagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_tag_key` after provisioning.\n"]
    pub fn resource_tag_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_tag_key", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_tag_value` after provisioning.\n"]
    pub fn resource_tag_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_tag_value", self.base))
    }
}

#[derive(Serialize)]
pub struct RbinRule_LockConfiguration_UnlockDelay {
    unlock_delay_unit: terrars::PrimField<std::string::String>,
    unlock_delay_value: terrars::PrimField<f64>,
}

impl RbinRule_LockConfiguration_UnlockDelay { }

impl terrars::ToListMappable for RbinRule_LockConfiguration_UnlockDelay {
    type O = terrars::BlockAssignable<RbinRule_LockConfiguration_UnlockDelay>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRbinRule_LockConfiguration_UnlockDelay {
    #[doc = ""]
    pub unlock_delay_unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub unlock_delay_value: terrars::PrimField<f64>,
}

impl BuildRbinRule_LockConfiguration_UnlockDelay {
    pub fn build(self) -> RbinRule_LockConfiguration_UnlockDelay {
        RbinRule_LockConfiguration_UnlockDelay {
            unlock_delay_unit: self.unlock_delay_unit,
            unlock_delay_value: self.unlock_delay_value,
        }
    }
}

#[allow(dead_code)]
pub struct RbinRule_LockConfiguration_UnlockDelayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RbinRule_LockConfiguration_UnlockDelayRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RbinRule_LockConfiguration_UnlockDelayRef {
        RbinRule_LockConfiguration_UnlockDelayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RbinRule_LockConfiguration_UnlockDelayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unlock_delay_unit` after provisioning.\n"]
    pub fn unlock_delay_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unlock_delay_unit", self.base))
    }

    #[doc = "Get a reference to the value of field `unlock_delay_value` after provisioning.\n"]
    pub fn unlock_delay_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unlock_delay_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct RbinRule_LockConfigurationDynamic {
    unlock_delay: std::option::Option<terrars::DynamicBlock<RbinRule_LockConfiguration_UnlockDelay>>,
}

#[derive(Serialize)]
pub struct RbinRule_LockConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unlock_delay: std::option::Option<std::vec::Vec<RbinRule_LockConfiguration_UnlockDelay>>,
    dynamic: RbinRule_LockConfigurationDynamic,
}

impl RbinRule_LockConfiguration {
    #[doc = "Set the field `unlock_delay`.\n"]
    pub fn set_unlock_delay(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RbinRule_LockConfiguration_UnlockDelay>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.unlock_delay = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.unlock_delay = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RbinRule_LockConfiguration {
    type O = terrars::BlockAssignable<RbinRule_LockConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRbinRule_LockConfiguration {}

impl BuildRbinRule_LockConfiguration {
    pub fn build(self) -> RbinRule_LockConfiguration {
        RbinRule_LockConfiguration {
            unlock_delay: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RbinRule_LockConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RbinRule_LockConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RbinRule_LockConfigurationRef {
        RbinRule_LockConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RbinRule_LockConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unlock_delay` after provisioning.\n"]
    pub fn unlock_delay(&self) -> terrars::ListRef<RbinRule_LockConfiguration_UnlockDelayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unlock_delay", self.base))
    }
}

#[derive(Serialize)]
pub struct RbinRule_ResourceTags {
    resource_tag_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_tag_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RbinRule_ResourceTags {
    #[doc = "Set the field `resource_tag_value`.\n"]
    pub fn set_resource_tag_value(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.resource_tag_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RbinRule_ResourceTags {
    type O = terrars::BlockAssignable<RbinRule_ResourceTags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRbinRule_ResourceTags {
    #[doc = ""]
    pub resource_tag_key: terrars::PrimField<std::string::String>,
}

impl BuildRbinRule_ResourceTags {
    pub fn build(self) -> RbinRule_ResourceTags {
        RbinRule_ResourceTags {
            resource_tag_key: self.resource_tag_key,
            resource_tag_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RbinRule_ResourceTagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RbinRule_ResourceTagsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RbinRule_ResourceTagsRef {
        RbinRule_ResourceTagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RbinRule_ResourceTagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_tag_key` after provisioning.\n"]
    pub fn resource_tag_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_tag_key", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_tag_value` after provisioning.\n"]
    pub fn resource_tag_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_tag_value", self.base))
    }
}

#[derive(Serialize)]
pub struct RbinRule_RetentionPeriod {
    retention_period_unit: terrars::PrimField<std::string::String>,
    retention_period_value: terrars::PrimField<f64>,
}

impl RbinRule_RetentionPeriod { }

impl terrars::ToListMappable for RbinRule_RetentionPeriod {
    type O = terrars::BlockAssignable<RbinRule_RetentionPeriod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRbinRule_RetentionPeriod {
    #[doc = ""]
    pub retention_period_unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub retention_period_value: terrars::PrimField<f64>,
}

impl BuildRbinRule_RetentionPeriod {
    pub fn build(self) -> RbinRule_RetentionPeriod {
        RbinRule_RetentionPeriod {
            retention_period_unit: self.retention_period_unit,
            retention_period_value: self.retention_period_value,
        }
    }
}

#[allow(dead_code)]
pub struct RbinRule_RetentionPeriodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RbinRule_RetentionPeriodRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RbinRule_RetentionPeriodRef {
        RbinRule_RetentionPeriodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RbinRule_RetentionPeriodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `retention_period_unit` after provisioning.\n"]
    pub fn retention_period_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period_unit", self.base))
    }

    #[doc = "Get a reference to the value of field `retention_period_value` after provisioning.\n"]
    pub fn retention_period_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retention_period_value", self.base))
    }
}

#[derive(Serialize)]
pub struct RbinRule_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RbinRule_Timeouts {
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

impl terrars::ToListMappable for RbinRule_Timeouts {
    type O = terrars::BlockAssignable<RbinRule_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRbinRule_Timeouts {}

impl BuildRbinRule_Timeouts {
    pub fn build(self) -> RbinRule_Timeouts {
        RbinRule_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RbinRule_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RbinRule_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RbinRule_TimeoutsRef {
        RbinRule_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RbinRule_TimeoutsRef {
    #[allow(dead_code)]
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
