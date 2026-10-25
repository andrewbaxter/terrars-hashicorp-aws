#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GameliftFleetData_ {
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
    build_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    ec2_instance_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fleet_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_groups: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_game_session_protection_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    script_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_configuration: std::option::Option<std::vec::Vec<GameliftFleet_CertificateConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_inbound_permission: std::option::Option<std::vec::Vec<GameliftFleet_Ec2InboundPermission>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_creation_limit_policy: std::option::Option<std::vec::Vec<GameliftFleet_ResourceCreationLimitPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_configuration: std::option::Option<std::vec::Vec<GameliftFleet_RuntimeConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<GameliftFleet_Timeouts>,
    dynamic: GameliftFleetDynamic,
}

struct GameliftFleet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GameliftFleetData_>,
}

#[derive(Clone)]
pub struct GameliftFleet(std::rc::Rc<GameliftFleet_>);

impl GameliftFleet {
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

    #[doc = "Set the field `build_id`.\n"]
    pub fn set_build_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().build_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fleet_type`.\n"]
    pub fn set_fleet_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().fleet_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_role_arn`.\n"]
    pub fn set_instance_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().instance_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metric_groups`.\n"]
    pub fn set_metric_groups(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().metric_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `new_game_session_protection_policy`.\n"]
    pub fn set_new_game_session_protection_policy(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().new_game_session_protection_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `script_id`.\n"]
    pub fn set_script_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().script_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `certificate_configuration`.\n"]
    pub fn set_certificate_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GameliftFleet_CertificateConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().certificate_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.certificate_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ec2_inbound_permission`.\n"]
    pub fn set_ec2_inbound_permission(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GameliftFleet_Ec2InboundPermission>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ec2_inbound_permission = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ec2_inbound_permission = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_creation_limit_policy`.\n"]
    pub fn set_resource_creation_limit_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GameliftFleet_ResourceCreationLimitPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_creation_limit_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_creation_limit_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `runtime_configuration`.\n"]
    pub fn set_runtime_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GameliftFleet_RuntimeConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().runtime_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.runtime_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<GameliftFleet_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_arn` after provisioning.\n"]
    pub fn build_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_id` after provisioning.\n"]
    pub fn build_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ec2_instance_type` after provisioning.\n"]
    pub fn ec2_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ec2_instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet_type` after provisioning.\n"]
    pub fn fleet_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fleet_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_role_arn` after provisioning.\n"]
    pub fn instance_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_paths` after provisioning.\n"]
    pub fn log_paths(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_paths", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_groups` after provisioning.\n"]
    pub fn metric_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `new_game_session_protection_policy` after provisioning.\n"]
    pub fn new_game_session_protection_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.new_game_session_protection_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `operating_system` after provisioning.\n"]
    pub fn operating_system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operating_system", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `script_arn` after provisioning.\n"]
    pub fn script_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `script_id` after provisioning.\n"]
    pub fn script_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_configuration` after provisioning.\n"]
    pub fn certificate_configuration(&self) -> terrars::ListRef<GameliftFleet_CertificateConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_creation_limit_policy` after provisioning.\n"]
    pub fn resource_creation_limit_policy(&self) -> terrars::ListRef<GameliftFleet_ResourceCreationLimitPolicyRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.resource_creation_limit_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `runtime_configuration` after provisioning.\n"]
    pub fn runtime_configuration(&self) -> terrars::ListRef<GameliftFleet_RuntimeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GameliftFleet_TimeoutsRef {
        GameliftFleet_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for GameliftFleet {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GameliftFleet { }

impl terrars::ToListMappable for GameliftFleet {
    type O = terrars::ListRef<GameliftFleetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GameliftFleet_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_gamelift_fleet".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGameliftFleet {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub ec2_instance_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildGameliftFleet {
    pub fn build(self, stack: &mut terrars::Stack) -> GameliftFleet {
        let out = GameliftFleet(std::rc::Rc::new(GameliftFleet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GameliftFleetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                build_id: core::default::Default::default(),
                description: core::default::Default::default(),
                ec2_instance_type: self.ec2_instance_type,
                fleet_type: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_role_arn: core::default::Default::default(),
                metric_groups: core::default::Default::default(),
                name: self.name,
                new_game_session_protection_policy: core::default::Default::default(),
                region: core::default::Default::default(),
                script_id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                certificate_configuration: core::default::Default::default(),
                ec2_inbound_permission: core::default::Default::default(),
                resource_creation_limit_policy: core::default::Default::default(),
                runtime_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GameliftFleetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GameliftFleetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GameliftFleetRef {
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

    #[doc = "Get a reference to the value of field `build_arn` after provisioning.\n"]
    pub fn build_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_id` after provisioning.\n"]
    pub fn build_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ec2_instance_type` after provisioning.\n"]
    pub fn ec2_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ec2_instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet_type` after provisioning.\n"]
    pub fn fleet_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fleet_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_role_arn` after provisioning.\n"]
    pub fn instance_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_paths` after provisioning.\n"]
    pub fn log_paths(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_paths", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_groups` after provisioning.\n"]
    pub fn metric_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `new_game_session_protection_policy` after provisioning.\n"]
    pub fn new_game_session_protection_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.new_game_session_protection_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `operating_system` after provisioning.\n"]
    pub fn operating_system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operating_system", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `script_arn` after provisioning.\n"]
    pub fn script_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `script_id` after provisioning.\n"]
    pub fn script_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.script_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_configuration` after provisioning.\n"]
    pub fn certificate_configuration(&self) -> terrars::ListRef<GameliftFleet_CertificateConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_creation_limit_policy` after provisioning.\n"]
    pub fn resource_creation_limit_policy(&self) -> terrars::ListRef<GameliftFleet_ResourceCreationLimitPolicyRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.resource_creation_limit_policy", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `runtime_configuration` after provisioning.\n"]
    pub fn runtime_configuration(&self) -> terrars::ListRef<GameliftFleet_RuntimeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> GameliftFleet_TimeoutsRef {
        GameliftFleet_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct GameliftFleetDynamic {
    certificate_configuration: std::option::Option<terrars::DynamicBlock<GameliftFleet_CertificateConfiguration>>,
    ec2_inbound_permission: std::option::Option<terrars::DynamicBlock<GameliftFleet_Ec2InboundPermission>>,
    resource_creation_limit_policy: std
    ::option
    ::Option<terrars::DynamicBlock<GameliftFleet_ResourceCreationLimitPolicy>>,
    runtime_configuration: std::option::Option<terrars::DynamicBlock<GameliftFleet_RuntimeConfiguration>>,
}

#[derive(Serialize)]
pub struct GameliftFleet_CertificateConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GameliftFleet_CertificateConfiguration {
    #[doc = "Set the field `certificate_type`.\n"]
    pub fn set_certificate_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.certificate_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GameliftFleet_CertificateConfiguration {
    type O = terrars::BlockAssignable<GameliftFleet_CertificateConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftFleet_CertificateConfiguration {}

impl BuildGameliftFleet_CertificateConfiguration {
    pub fn build(self) -> GameliftFleet_CertificateConfiguration {
        GameliftFleet_CertificateConfiguration { certificate_type: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct GameliftFleet_CertificateConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GameliftFleet_CertificateConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GameliftFleet_CertificateConfigurationRef {
        GameliftFleet_CertificateConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftFleet_CertificateConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_type` after provisioning.\n"]
    pub fn certificate_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_type", self.base))
    }
}

#[derive(Serialize)]
pub struct GameliftFleet_Ec2InboundPermission {
    from_port: terrars::PrimField<f64>,
    ip_range: terrars::PrimField<std::string::String>,
    protocol: terrars::PrimField<std::string::String>,
    to_port: terrars::PrimField<f64>,
}

impl GameliftFleet_Ec2InboundPermission { }

impl terrars::ToListMappable for GameliftFleet_Ec2InboundPermission {
    type O = terrars::BlockAssignable<GameliftFleet_Ec2InboundPermission>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftFleet_Ec2InboundPermission {
    #[doc = ""]
    pub from_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub ip_range: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub protocol: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub to_port: terrars::PrimField<f64>,
}

impl BuildGameliftFleet_Ec2InboundPermission {
    pub fn build(self) -> GameliftFleet_Ec2InboundPermission {
        GameliftFleet_Ec2InboundPermission {
            from_port: self.from_port,
            ip_range: self.ip_range,
            protocol: self.protocol,
            to_port: self.to_port,
        }
    }
}

#[allow(dead_code)]
pub struct GameliftFleet_Ec2InboundPermissionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GameliftFleet_Ec2InboundPermissionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GameliftFleet_Ec2InboundPermissionRef {
        GameliftFleet_Ec2InboundPermissionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftFleet_Ec2InboundPermissionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_range` after provisioning.\n"]
    pub fn ip_range(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_range", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize)]
pub struct GameliftFleet_ResourceCreationLimitPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    new_game_sessions_per_creator: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_period_in_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl GameliftFleet_ResourceCreationLimitPolicy {
    #[doc = "Set the field `new_game_sessions_per_creator`.\n"]
    pub fn set_new_game_sessions_per_creator(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.new_game_sessions_per_creator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_period_in_minutes`.\n"]
    pub fn set_policy_period_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.policy_period_in_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GameliftFleet_ResourceCreationLimitPolicy {
    type O = terrars::BlockAssignable<GameliftFleet_ResourceCreationLimitPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftFleet_ResourceCreationLimitPolicy {}

impl BuildGameliftFleet_ResourceCreationLimitPolicy {
    pub fn build(self) -> GameliftFleet_ResourceCreationLimitPolicy {
        GameliftFleet_ResourceCreationLimitPolicy {
            new_game_sessions_per_creator: core::default::Default::default(),
            policy_period_in_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GameliftFleet_ResourceCreationLimitPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GameliftFleet_ResourceCreationLimitPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GameliftFleet_ResourceCreationLimitPolicyRef {
        GameliftFleet_ResourceCreationLimitPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftFleet_ResourceCreationLimitPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `new_game_sessions_per_creator` after provisioning.\n"]
    pub fn new_game_sessions_per_creator(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.new_game_sessions_per_creator", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_period_in_minutes` after provisioning.\n"]
    pub fn policy_period_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_period_in_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct GameliftFleet_RuntimeConfiguration_ServerProcess {
    concurrent_executions: terrars::PrimField<f64>,
    launch_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GameliftFleet_RuntimeConfiguration_ServerProcess {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.parameters = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GameliftFleet_RuntimeConfiguration_ServerProcess {
    type O = terrars::BlockAssignable<GameliftFleet_RuntimeConfiguration_ServerProcess>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftFleet_RuntimeConfiguration_ServerProcess {
    #[doc = ""]
    pub concurrent_executions: terrars::PrimField<f64>,
    #[doc = ""]
    pub launch_path: terrars::PrimField<std::string::String>,
}

impl BuildGameliftFleet_RuntimeConfiguration_ServerProcess {
    pub fn build(self) -> GameliftFleet_RuntimeConfiguration_ServerProcess {
        GameliftFleet_RuntimeConfiguration_ServerProcess {
            concurrent_executions: self.concurrent_executions,
            launch_path: self.launch_path,
            parameters: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GameliftFleet_RuntimeConfiguration_ServerProcessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GameliftFleet_RuntimeConfiguration_ServerProcessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> GameliftFleet_RuntimeConfiguration_ServerProcessRef {
        GameliftFleet_RuntimeConfiguration_ServerProcessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftFleet_RuntimeConfiguration_ServerProcessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `concurrent_executions` after provisioning.\n"]
    pub fn concurrent_executions(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.concurrent_executions", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_path` after provisioning.\n"]
    pub fn launch_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_path", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct GameliftFleet_RuntimeConfigurationDynamic {
    server_process: std::option::Option<terrars::DynamicBlock<GameliftFleet_RuntimeConfiguration_ServerProcess>>,
}

#[derive(Serialize)]
pub struct GameliftFleet_RuntimeConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    game_session_activation_timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_concurrent_game_session_activations: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_process: std::option::Option<std::vec::Vec<GameliftFleet_RuntimeConfiguration_ServerProcess>>,
    dynamic: GameliftFleet_RuntimeConfigurationDynamic,
}

impl GameliftFleet_RuntimeConfiguration {
    #[doc = "Set the field `game_session_activation_timeout_seconds`.\n"]
    pub fn set_game_session_activation_timeout_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.game_session_activation_timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_concurrent_game_session_activations`.\n"]
    pub fn set_max_concurrent_game_session_activations(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.max_concurrent_game_session_activations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_process`.\n"]
    pub fn set_server_process(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GameliftFleet_RuntimeConfiguration_ServerProcess>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.server_process = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.server_process = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GameliftFleet_RuntimeConfiguration {
    type O = terrars::BlockAssignable<GameliftFleet_RuntimeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftFleet_RuntimeConfiguration {}

impl BuildGameliftFleet_RuntimeConfiguration {
    pub fn build(self) -> GameliftFleet_RuntimeConfiguration {
        GameliftFleet_RuntimeConfiguration {
            game_session_activation_timeout_seconds: core::default::Default::default(),
            max_concurrent_game_session_activations: core::default::Default::default(),
            server_process: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GameliftFleet_RuntimeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GameliftFleet_RuntimeConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GameliftFleet_RuntimeConfigurationRef {
        GameliftFleet_RuntimeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftFleet_RuntimeConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `game_session_activation_timeout_seconds` after provisioning.\n"]
    pub fn game_session_activation_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.game_session_activation_timeout_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `max_concurrent_game_session_activations` after provisioning.\n"]
    pub fn max_concurrent_game_session_activations(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_concurrent_game_session_activations", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `server_process` after provisioning.\n"]
    pub fn server_process(&self) -> terrars::ListRef<GameliftFleet_RuntimeConfiguration_ServerProcessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_process", self.base))
    }
}

#[derive(Serialize)]
pub struct GameliftFleet_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl GameliftFleet_Timeouts {
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

impl terrars::ToListMappable for GameliftFleet_Timeouts {
    type O = terrars::BlockAssignable<GameliftFleet_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGameliftFleet_Timeouts {}

impl BuildGameliftFleet_Timeouts {
    pub fn build(self) -> GameliftFleet_Timeouts {
        GameliftFleet_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GameliftFleet_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GameliftFleet_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GameliftFleet_TimeoutsRef {
        GameliftFleet_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GameliftFleet_TimeoutsRef {
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
}
