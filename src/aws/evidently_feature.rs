use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EvidentlyFeatureData {
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
    default_variation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    entity_overrides: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    project: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EvidentlyFeatureTimeoutsEl>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    variations: std::option::Option<std::vec::Vec<EvidentlyFeatureVariationsEl>>,
    dynamic: EvidentlyFeatureDynamic,
}

struct EvidentlyFeature_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EvidentlyFeatureData>,
}

#[derive(Clone)]
pub struct EvidentlyFeature(std::rc::Rc<EvidentlyFeature_>);

impl EvidentlyFeature {
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

    #[doc = "Set the field `default_variation`.\n"]
    pub fn set_default_variation(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_variation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `entity_overrides`.\n"]
    pub fn set_entity_overrides(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().entity_overrides = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluation_strategy`.\n"]
    pub fn set_evaluation_strategy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().evaluation_strategy = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EvidentlyFeatureTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `variations`.\n"]
    pub fn set_variations(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EvidentlyFeatureVariationsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().variations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.variations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_variation` after provisioning.\n"]
    pub fn default_variation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_variation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entity_overrides` after provisioning.\n"]
    pub fn entity_overrides(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.entity_overrides", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluation_rules` after provisioning.\n"]
    pub fn evaluation_rules(&self) -> terrars::SetRef<EvidentlyFeatureEvaluationRulesElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.evaluation_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluation_strategy` after provisioning.\n"]
    pub fn evaluation_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `value_type` after provisioning.\n"]
    pub fn value_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EvidentlyFeatureTimeoutsElRef {
        EvidentlyFeatureTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for EvidentlyFeature {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EvidentlyFeature { }

impl terrars::ToListMappable for EvidentlyFeature {
    type O = terrars::ListRef<EvidentlyFeatureRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EvidentlyFeature_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_evidently_feature".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEvidentlyFeature {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub project: terrars::PrimField<std::string::String>,
}

impl BuildEvidentlyFeature {
    pub fn build(self, stack: &mut terrars::Stack) -> EvidentlyFeature {
        let out = EvidentlyFeature(std::rc::Rc::new(EvidentlyFeature_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EvidentlyFeatureData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                default_variation: core::default::Default::default(),
                description: core::default::Default::default(),
                entity_overrides: core::default::Default::default(),
                evaluation_strategy: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: self.project,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                variations: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EvidentlyFeatureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EvidentlyFeatureRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EvidentlyFeatureRef {
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

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_variation` after provisioning.\n"]
    pub fn default_variation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_variation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `entity_overrides` after provisioning.\n"]
    pub fn entity_overrides(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.entity_overrides", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluation_rules` after provisioning.\n"]
    pub fn evaluation_rules(&self) -> terrars::SetRef<EvidentlyFeatureEvaluationRulesElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.evaluation_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluation_strategy` after provisioning.\n"]
    pub fn evaluation_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `value_type` after provisioning.\n"]
    pub fn value_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EvidentlyFeatureTimeoutsElRef {
        EvidentlyFeatureTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EvidentlyFeatureEvaluationRulesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EvidentlyFeatureEvaluationRulesEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EvidentlyFeatureEvaluationRulesEl {
    type O = terrars::BlockAssignable<EvidentlyFeatureEvaluationRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyFeatureEvaluationRulesEl {}

impl BuildEvidentlyFeatureEvaluationRulesEl {
    pub fn build(self) -> EvidentlyFeatureEvaluationRulesEl {
        EvidentlyFeatureEvaluationRulesEl {
            name: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct EvidentlyFeatureEvaluationRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EvidentlyFeatureEvaluationRulesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EvidentlyFeatureEvaluationRulesElRef {
        EvidentlyFeatureEvaluationRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyFeatureEvaluationRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct EvidentlyFeatureTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EvidentlyFeatureTimeoutsEl {
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

impl terrars::ToListMappable for EvidentlyFeatureTimeoutsEl {
    type O = terrars::BlockAssignable<EvidentlyFeatureTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyFeatureTimeoutsEl {}

impl BuildEvidentlyFeatureTimeoutsEl {
    pub fn build(self) -> EvidentlyFeatureTimeoutsEl {
        EvidentlyFeatureTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct EvidentlyFeatureTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EvidentlyFeatureTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EvidentlyFeatureTimeoutsElRef {
        EvidentlyFeatureTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyFeatureTimeoutsElRef {
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

#[derive(Serialize)]
pub struct EvidentlyFeatureVariationsElValueEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bool_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    double_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    long_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EvidentlyFeatureVariationsElValueEl {
    #[doc = "Set the field `bool_value`.\n"]
    pub fn set_bool_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bool_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `double_value`.\n"]
    pub fn set_double_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.double_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `long_value`.\n"]
    pub fn set_long_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.long_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_value`.\n"]
    pub fn set_string_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.string_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EvidentlyFeatureVariationsElValueEl {
    type O = terrars::BlockAssignable<EvidentlyFeatureVariationsElValueEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyFeatureVariationsElValueEl {}

impl BuildEvidentlyFeatureVariationsElValueEl {
    pub fn build(self) -> EvidentlyFeatureVariationsElValueEl {
        EvidentlyFeatureVariationsElValueEl {
            bool_value: core::default::Default::default(),
            double_value: core::default::Default::default(),
            long_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
        }
    }
}

pub struct EvidentlyFeatureVariationsElValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EvidentlyFeatureVariationsElValueElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EvidentlyFeatureVariationsElValueElRef {
        EvidentlyFeatureVariationsElValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyFeatureVariationsElValueElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bool_value` after provisioning.\n"]
    pub fn bool_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bool_value", self.base))
    }

    #[doc = "Get a reference to the value of field `double_value` after provisioning.\n"]
    pub fn double_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.double_value", self.base))
    }

    #[doc = "Get a reference to the value of field `long_value` after provisioning.\n"]
    pub fn long_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.long_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_value` after provisioning.\n"]
    pub fn string_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct EvidentlyFeatureVariationsElDynamic {
    value: std::option::Option<terrars::DynamicBlock<EvidentlyFeatureVariationsElValueEl>>,
}

#[derive(Serialize)]
pub struct EvidentlyFeatureVariationsEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<std::vec::Vec<EvidentlyFeatureVariationsElValueEl>>,
    dynamic: EvidentlyFeatureVariationsElDynamic,
}

impl EvidentlyFeatureVariationsEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EvidentlyFeatureVariationsElValueEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EvidentlyFeatureVariationsEl {
    type O = terrars::BlockAssignable<EvidentlyFeatureVariationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyFeatureVariationsEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildEvidentlyFeatureVariationsEl {
    pub fn build(self) -> EvidentlyFeatureVariationsEl {
        EvidentlyFeatureVariationsEl {
            name: self.name,
            value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EvidentlyFeatureVariationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EvidentlyFeatureVariationsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EvidentlyFeatureVariationsElRef {
        EvidentlyFeatureVariationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyFeatureVariationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::ListRef<EvidentlyFeatureVariationsElValueElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct EvidentlyFeatureDynamic {
    variations: std::option::Option<terrars::DynamicBlock<EvidentlyFeatureVariationsEl>>,
}
