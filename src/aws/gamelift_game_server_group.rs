use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GameliftGameServerGroupData {
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
    balancing_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    game_server_group_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    game_server_protection_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    max_size: terrars::PrimField<f64>,
    min_size: terrars::PrimField<f64>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_subnets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_scaling_policy: std::option::Option<std::vec::Vec<GameliftGameServerGroupAutoScalingPolicyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_definition: std::option::Option<std::vec::Vec<GameliftGameServerGroupInstanceDefinitionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template: std::option::Option<std::vec::Vec<GameliftGameServerGroupLaunchTemplateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GameliftGameServerGroupTimeoutsEl>,
    dynamic: GameliftGameServerGroupDynamic,
}

struct GameliftGameServerGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GameliftGameServerGroupData>,
}

#[derive(Clone)]
pub struct GameliftGameServerGroup(std::rc::Rc<GameliftGameServerGroup_>);

impl GameliftGameServerGroup {
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

    #[doc = "Set the field `balancing_strategy`.\n"]
    pub fn set_balancing_strategy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().balancing_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `game_server_protection_policy`.\n"]
    pub fn set_game_server_protection_policy(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().game_server_protection_policy = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `vpc_subnets`.\n"]
    pub fn set_vpc_subnets(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().vpc_subnets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_scaling_policy`.\n"]
    pub fn set_auto_scaling_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GameliftGameServerGroupAutoScalingPolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_scaling_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_scaling_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_definition`.\n"]
    pub fn set_instance_definition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GameliftGameServerGroupInstanceDefinitionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_definition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `launch_template`.\n"]
    pub fn set_launch_template(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GameliftGameServerGroupLaunchTemplateEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().launch_template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.launch_template = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GameliftGameServerGroupTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_scaling_group_arn` after provisioning.\n"]
    pub fn auto_scaling_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_scaling_group_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `balancing_strategy` after provisioning.\n"]
    pub fn balancing_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.balancing_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `game_server_group_name` after provisioning.\n"]
    pub fn game_server_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.game_server_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `game_server_protection_policy` after provisioning.\n"]
    pub fn game_server_protection_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.game_server_protection_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_size` after provisioning.\n"]
    pub fn max_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_size` after provisioning.\n"]
    pub fn min_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_size", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `vpc_subnets` after provisioning.\n"]
    pub fn vpc_subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_subnets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_scaling_policy` after provisioning.\n"]
    pub fn auto_scaling_policy(&self) -> terrars::ListRef<GameliftGameServerGroupAutoScalingPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_scaling_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> terrars::ListRef<GameliftGameServerGroupLaunchTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GameliftGameServerGroupTimeoutsElRef {
        GameliftGameServerGroupTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for GameliftGameServerGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GameliftGameServerGroup { }

impl terrars::ToListMappable for GameliftGameServerGroup {
    type O = terrars::ListRef<GameliftGameServerGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GameliftGameServerGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_gamelift_game_server_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGameliftGameServerGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub game_server_group_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub max_size: terrars::PrimField<f64>,
    #[doc = ""]
    pub min_size: terrars::PrimField<f64>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildGameliftGameServerGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> GameliftGameServerGroup {
        let out = GameliftGameServerGroup(std::rc::Rc::new(GameliftGameServerGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GameliftGameServerGroupData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                balancing_strategy: core::default::Default::default(),
                game_server_group_name: self.game_server_group_name,
                game_server_protection_policy: core::default::Default::default(),
                id: core::default::Default::default(),
                max_size: self.max_size,
                min_size: self.min_size,
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_subnets: core::default::Default::default(),
                auto_scaling_policy: core::default::Default::default(),
                instance_definition: core::default::Default::default(),
                launch_template: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GameliftGameServerGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GameliftGameServerGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GameliftGameServerGroupRef {
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

    #[doc = "Get a reference to the value of field `auto_scaling_group_arn` after provisioning.\n"]
    pub fn auto_scaling_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_scaling_group_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `balancing_strategy` after provisioning.\n"]
    pub fn balancing_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.balancing_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `game_server_group_name` after provisioning.\n"]
    pub fn game_server_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.game_server_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `game_server_protection_policy` after provisioning.\n"]
    pub fn game_server_protection_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.game_server_protection_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_size` after provisioning.\n"]
    pub fn max_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `min_size` after provisioning.\n"]
    pub fn min_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_size", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `vpc_subnets` after provisioning.\n"]
    pub fn vpc_subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_subnets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_scaling_policy` after provisioning.\n"]
    pub fn auto_scaling_policy(&self) -> terrars::ListRef<GameliftGameServerGroupAutoScalingPolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_scaling_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> terrars::ListRef<GameliftGameServerGroupLaunchTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GameliftGameServerGroupTimeoutsElRef {
        GameliftGameServerGroupTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl {
    target_value: terrars::PrimField<f64>,
}

impl GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl { }

impl terrars::ToListMappable for GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl {
    type O = terrars::BlockAssignable<GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl {
    #[doc = ""]
    pub target_value: terrars::PrimField<f64>,
}

impl BuildGameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl {
    pub fn build(self) -> GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl {
        GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl { target_value: self.target_value }
    }
}

pub struct GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationElRef {
        GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_value` after provisioning.\n"]
    pub fn target_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct GameliftGameServerGroupAutoScalingPolicyElDynamic {
    target_tracking_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl>>,
}

#[derive(Serialize)]
pub struct GameliftGameServerGroupAutoScalingPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    estimated_instance_warmup: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_tracking_configuration: std
    ::option
    ::Option<std::vec::Vec<GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl>>,
    dynamic: GameliftGameServerGroupAutoScalingPolicyElDynamic,
}

impl GameliftGameServerGroupAutoScalingPolicyEl {
    #[doc = "Set the field `estimated_instance_warmup`.\n"]
    pub fn set_estimated_instance_warmup(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.estimated_instance_warmup = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_tracking_configuration`.\n"]
    pub fn set_target_tracking_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_tracking_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_tracking_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GameliftGameServerGroupAutoScalingPolicyEl {
    type O = terrars::BlockAssignable<GameliftGameServerGroupAutoScalingPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftGameServerGroupAutoScalingPolicyEl {}

impl BuildGameliftGameServerGroupAutoScalingPolicyEl {
    pub fn build(self) -> GameliftGameServerGroupAutoScalingPolicyEl {
        GameliftGameServerGroupAutoScalingPolicyEl {
            estimated_instance_warmup: core::default::Default::default(),
            target_tracking_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct GameliftGameServerGroupAutoScalingPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GameliftGameServerGroupAutoScalingPolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GameliftGameServerGroupAutoScalingPolicyElRef {
        GameliftGameServerGroupAutoScalingPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftGameServerGroupAutoScalingPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `estimated_instance_warmup` after provisioning.\n"]
    pub fn estimated_instance_warmup(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.estimated_instance_warmup", self.base))
    }

    #[doc = "Get a reference to the value of field `target_tracking_configuration` after provisioning.\n"]
    pub fn target_tracking_configuration(
        &self,
    ) -> terrars::ListRef<GameliftGameServerGroupAutoScalingPolicyElTargetTrackingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_tracking_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct GameliftGameServerGroupInstanceDefinitionEl {
    instance_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_capacity: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GameliftGameServerGroupInstanceDefinitionEl {
    #[doc = "Set the field `weighted_capacity`.\n"]
    pub fn set_weighted_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.weighted_capacity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GameliftGameServerGroupInstanceDefinitionEl {
    type O = terrars::BlockAssignable<GameliftGameServerGroupInstanceDefinitionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftGameServerGroupInstanceDefinitionEl {
    #[doc = ""]
    pub instance_type: terrars::PrimField<std::string::String>,
}

impl BuildGameliftGameServerGroupInstanceDefinitionEl {
    pub fn build(self) -> GameliftGameServerGroupInstanceDefinitionEl {
        GameliftGameServerGroupInstanceDefinitionEl {
            instance_type: self.instance_type,
            weighted_capacity: core::default::Default::default(),
        }
    }
}

pub struct GameliftGameServerGroupInstanceDefinitionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GameliftGameServerGroupInstanceDefinitionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GameliftGameServerGroupInstanceDefinitionElRef {
        GameliftGameServerGroupInstanceDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftGameServerGroupInstanceDefinitionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_capacity` after provisioning.\n"]
    pub fn weighted_capacity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weighted_capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct GameliftGameServerGroupLaunchTemplateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GameliftGameServerGroupLaunchTemplateEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GameliftGameServerGroupLaunchTemplateEl {
    type O = terrars::BlockAssignable<GameliftGameServerGroupLaunchTemplateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftGameServerGroupLaunchTemplateEl {}

impl BuildGameliftGameServerGroupLaunchTemplateEl {
    pub fn build(self) -> GameliftGameServerGroupLaunchTemplateEl {
        GameliftGameServerGroupLaunchTemplateEl {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct GameliftGameServerGroupLaunchTemplateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GameliftGameServerGroupLaunchTemplateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GameliftGameServerGroupLaunchTemplateElRef {
        GameliftGameServerGroupLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftGameServerGroupLaunchTemplateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct GameliftGameServerGroupTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GameliftGameServerGroupTimeoutsEl {
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
}

impl terrars::ToListMappable for GameliftGameServerGroupTimeoutsEl {
    type O = terrars::BlockAssignable<GameliftGameServerGroupTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftGameServerGroupTimeoutsEl {}

impl BuildGameliftGameServerGroupTimeoutsEl {
    pub fn build(self) -> GameliftGameServerGroupTimeoutsEl {
        GameliftGameServerGroupTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct GameliftGameServerGroupTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GameliftGameServerGroupTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GameliftGameServerGroupTimeoutsElRef {
        GameliftGameServerGroupTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftGameServerGroupTimeoutsElRef {
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
}

#[derive(Serialize, Default)]
struct GameliftGameServerGroupDynamic {
    auto_scaling_policy: std::option::Option<terrars::DynamicBlock<GameliftGameServerGroupAutoScalingPolicyEl>>,
    instance_definition: std::option::Option<terrars::DynamicBlock<GameliftGameServerGroupInstanceDefinitionEl>>,
    launch_template: std::option::Option<terrars::DynamicBlock<GameliftGameServerGroupLaunchTemplateEl>>,
}
