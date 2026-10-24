use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FisExperimentTemplateData {
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
    description: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<FisExperimentTemplateActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stop_condition: std::option::Option<std::vec::Vec<FisExperimentTemplateStopConditionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<std::vec::Vec<FisExperimentTemplateTargetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FisExperimentTemplateTimeoutsEl>,
    dynamic: FisExperimentTemplateDynamic,
}

struct FisExperimentTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FisExperimentTemplateData>,
}

#[derive(Clone)]
pub struct FisExperimentTemplate(std::rc::Rc<FisExperimentTemplate_>);

impl FisExperimentTemplate {
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

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FisExperimentTemplateActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stop_condition`.\n"]
    pub fn set_stop_condition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FisExperimentTemplateStopConditionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stop_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stop_condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FisExperimentTemplateTargetEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FisExperimentTemplateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FisExperimentTemplateTimeoutsElRef {
        FisExperimentTemplateTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for FisExperimentTemplate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FisExperimentTemplate { }

impl terrars::ToListMappable for FisExperimentTemplate {
    type O = terrars::ListRef<FisExperimentTemplateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FisExperimentTemplate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_fis_experiment_template".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFisExperimentTemplate {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildFisExperimentTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> FisExperimentTemplate {
        let out = FisExperimentTemplate(std::rc::Rc::new(FisExperimentTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FisExperimentTemplateData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: self.description,
                id: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                action: core::default::Default::default(),
                stop_condition: core::default::Default::default(),
                target: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FisExperimentTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FisExperimentTemplateRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FisExperimentTemplateTimeoutsElRef {
        FisExperimentTemplateTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FisExperimentTemplateActionElParameterEl {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl FisExperimentTemplateActionElParameterEl { }

impl terrars::ToListMappable for FisExperimentTemplateActionElParameterEl {
    type O = terrars::BlockAssignable<FisExperimentTemplateActionElParameterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplateActionElParameterEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildFisExperimentTemplateActionElParameterEl {
    pub fn build(self) -> FisExperimentTemplateActionElParameterEl {
        FisExperimentTemplateActionElParameterEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct FisExperimentTemplateActionElParameterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplateActionElParameterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FisExperimentTemplateActionElParameterElRef {
        FisExperimentTemplateActionElParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplateActionElParameterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct FisExperimentTemplateActionElTargetEl {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl FisExperimentTemplateActionElTargetEl { }

impl terrars::ToListMappable for FisExperimentTemplateActionElTargetEl {
    type O = terrars::BlockAssignable<FisExperimentTemplateActionElTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplateActionElTargetEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildFisExperimentTemplateActionElTargetEl {
    pub fn build(self) -> FisExperimentTemplateActionElTargetEl {
        FisExperimentTemplateActionElTargetEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct FisExperimentTemplateActionElTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplateActionElTargetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FisExperimentTemplateActionElTargetElRef {
        FisExperimentTemplateActionElTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplateActionElTargetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct FisExperimentTemplateActionElDynamic {
    parameter: std::option::Option<terrars::DynamicBlock<FisExperimentTemplateActionElParameterEl>>,
    target: std::option::Option<terrars::DynamicBlock<FisExperimentTemplateActionElTargetEl>>,
}

#[derive(Serialize)]
pub struct FisExperimentTemplateActionEl {
    action_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_after: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<std::vec::Vec<FisExperimentTemplateActionElParameterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<std::vec::Vec<FisExperimentTemplateActionElTargetEl>>,
    dynamic: FisExperimentTemplateActionElDynamic,
}

impl FisExperimentTemplateActionEl {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_after`.\n"]
    pub fn set_start_after(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.start_after = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FisExperimentTemplateActionElParameterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FisExperimentTemplateActionElTargetEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FisExperimentTemplateActionEl {
    type O = terrars::BlockAssignable<FisExperimentTemplateActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplateActionEl {
    #[doc = ""]
    pub action_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildFisExperimentTemplateActionEl {
    pub fn build(self) -> FisExperimentTemplateActionEl {
        FisExperimentTemplateActionEl {
            action_id: self.action_id,
            description: core::default::Default::default(),
            name: self.name,
            start_after: core::default::Default::default(),
            parameter: core::default::Default::default(),
            target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct FisExperimentTemplateActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplateActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FisExperimentTemplateActionElRef {
        FisExperimentTemplateActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplateActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_id` after provisioning.\n"]
    pub fn action_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_id", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `start_after` after provisioning.\n"]
    pub fn start_after(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.start_after", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::ListRef<FisExperimentTemplateActionElTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target", self.base))
    }
}

#[derive(Serialize)]
pub struct FisExperimentTemplateStopConditionEl {
    source: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FisExperimentTemplateStopConditionEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FisExperimentTemplateStopConditionEl {
    type O = terrars::BlockAssignable<FisExperimentTemplateStopConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplateStopConditionEl {
    #[doc = ""]
    pub source: terrars::PrimField<std::string::String>,
}

impl BuildFisExperimentTemplateStopConditionEl {
    pub fn build(self) -> FisExperimentTemplateStopConditionEl {
        FisExperimentTemplateStopConditionEl {
            source: self.source,
            value: core::default::Default::default(),
        }
    }
}

pub struct FisExperimentTemplateStopConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplateStopConditionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FisExperimentTemplateStopConditionElRef {
        FisExperimentTemplateStopConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplateStopConditionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct FisExperimentTemplateTargetElFilterEl {
    path: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl FisExperimentTemplateTargetElFilterEl { }

impl terrars::ToListMappable for FisExperimentTemplateTargetElFilterEl {
    type O = terrars::BlockAssignable<FisExperimentTemplateTargetElFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplateTargetElFilterEl {
    #[doc = ""]
    pub path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildFisExperimentTemplateTargetElFilterEl {
    pub fn build(self) -> FisExperimentTemplateTargetElFilterEl {
        FisExperimentTemplateTargetElFilterEl {
            path: self.path,
            values: self.values,
        }
    }
}

pub struct FisExperimentTemplateTargetElFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplateTargetElFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FisExperimentTemplateTargetElFilterElRef {
        FisExperimentTemplateTargetElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplateTargetElFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct FisExperimentTemplateTargetElResourceTagEl {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl FisExperimentTemplateTargetElResourceTagEl { }

impl terrars::ToListMappable for FisExperimentTemplateTargetElResourceTagEl {
    type O = terrars::BlockAssignable<FisExperimentTemplateTargetElResourceTagEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplateTargetElResourceTagEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildFisExperimentTemplateTargetElResourceTagEl {
    pub fn build(self) -> FisExperimentTemplateTargetElResourceTagEl {
        FisExperimentTemplateTargetElResourceTagEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct FisExperimentTemplateTargetElResourceTagElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplateTargetElResourceTagElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FisExperimentTemplateTargetElResourceTagElRef {
        FisExperimentTemplateTargetElResourceTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplateTargetElResourceTagElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct FisExperimentTemplateTargetElDynamic {
    filter: std::option::Option<terrars::DynamicBlock<FisExperimentTemplateTargetElFilterEl>>,
    resource_tag: std::option::Option<terrars::DynamicBlock<FisExperimentTemplateTargetElResourceTagEl>>,
}

#[derive(Serialize)]
pub struct FisExperimentTemplateTargetEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    resource_type: terrars::PrimField<std::string::String>,
    selection_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<FisExperimentTemplateTargetElFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_tag: std::option::Option<std::vec::Vec<FisExperimentTemplateTargetElResourceTagEl>>,
    dynamic: FisExperimentTemplateTargetElDynamic,
}

impl FisExperimentTemplateTargetEl {
    #[doc = "Set the field `resource_arns`.\n"]
    pub fn set_resource_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FisExperimentTemplateTargetElFilterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_tag`.\n"]
    pub fn set_resource_tag(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FisExperimentTemplateTargetElResourceTagEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_tag = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_tag = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FisExperimentTemplateTargetEl {
    type O = terrars::BlockAssignable<FisExperimentTemplateTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplateTargetEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub resource_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub selection_mode: terrars::PrimField<std::string::String>,
}

impl BuildFisExperimentTemplateTargetEl {
    pub fn build(self) -> FisExperimentTemplateTargetEl {
        FisExperimentTemplateTargetEl {
            name: self.name,
            resource_arns: core::default::Default::default(),
            resource_type: self.resource_type,
            selection_mode: self.selection_mode,
            filter: core::default::Default::default(),
            resource_tag: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct FisExperimentTemplateTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplateTargetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FisExperimentTemplateTargetElRef {
        FisExperimentTemplateTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplateTargetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_arns` after provisioning.\n"]
    pub fn resource_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resource_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }

    #[doc = "Get a reference to the value of field `selection_mode` after provisioning.\n"]
    pub fn selection_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.selection_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<FisExperimentTemplateTargetElFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize)]
pub struct FisExperimentTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FisExperimentTemplateTimeoutsEl {
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

impl terrars::ToListMappable for FisExperimentTemplateTimeoutsEl {
    type O = terrars::BlockAssignable<FisExperimentTemplateTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFisExperimentTemplateTimeoutsEl {}

impl BuildFisExperimentTemplateTimeoutsEl {
    pub fn build(self) -> FisExperimentTemplateTimeoutsEl {
        FisExperimentTemplateTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct FisExperimentTemplateTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FisExperimentTemplateTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FisExperimentTemplateTimeoutsElRef {
        FisExperimentTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FisExperimentTemplateTimeoutsElRef {
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
struct FisExperimentTemplateDynamic {
    action: std::option::Option<terrars::DynamicBlock<FisExperimentTemplateActionEl>>,
    stop_condition: std::option::Option<terrars::DynamicBlock<FisExperimentTemplateStopConditionEl>>,
    target: std::option::Option<terrars::DynamicBlock<FisExperimentTemplateTargetEl>>,
}
