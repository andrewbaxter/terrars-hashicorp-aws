use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OpsworksEcsClusterLayerData {
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
    auto_assign_elastic_ips: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_assign_public_ips: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_healing: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_configure_recipes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_deploy_recipes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_instance_profile_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_json: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_setup_recipes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_shutdown_recipes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_undeploy_recipes: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drain_elb_on_shutdown: std::option::Option<terrars::PrimField<bool>>,
    ecs_cluster_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elastic_load_balancer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    install_updates_on_boot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_shutdown_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    stack_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    system_packages: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_ebs_optimized_instances: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_configuration: std::option::Option<std::vec::Vec<OpsworksEcsClusterLayerCloudwatchConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_volume: std::option::Option<std::vec::Vec<OpsworksEcsClusterLayerEbsVolumeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_based_auto_scaling: std::option::Option<std::vec::Vec<OpsworksEcsClusterLayerLoadBasedAutoScalingEl>>,
    dynamic: OpsworksEcsClusterLayerDynamic,
}

struct OpsworksEcsClusterLayer_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OpsworksEcsClusterLayerData>,
}

#[derive(Clone)]
pub struct OpsworksEcsClusterLayer(std::rc::Rc<OpsworksEcsClusterLayer_>);

impl OpsworksEcsClusterLayer {
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

    #[doc = "Set the field `auto_assign_elastic_ips`.\n"]
    pub fn set_auto_assign_elastic_ips(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_assign_elastic_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_assign_public_ips`.\n"]
    pub fn set_auto_assign_public_ips(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_assign_public_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_healing`.\n"]
    pub fn set_auto_healing(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_healing = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_configure_recipes`.\n"]
    pub fn set_custom_configure_recipes(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().custom_configure_recipes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_deploy_recipes`.\n"]
    pub fn set_custom_deploy_recipes(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().custom_deploy_recipes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_instance_profile_arn`.\n"]
    pub fn set_custom_instance_profile_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().custom_instance_profile_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_json`.\n"]
    pub fn set_custom_json(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().custom_json = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_security_group_ids`.\n"]
    pub fn set_custom_security_group_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().custom_security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_setup_recipes`.\n"]
    pub fn set_custom_setup_recipes(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().custom_setup_recipes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_shutdown_recipes`.\n"]
    pub fn set_custom_shutdown_recipes(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().custom_shutdown_recipes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_undeploy_recipes`.\n"]
    pub fn set_custom_undeploy_recipes(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().custom_undeploy_recipes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `drain_elb_on_shutdown`.\n"]
    pub fn set_drain_elb_on_shutdown(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().drain_elb_on_shutdown = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `elastic_load_balancer`.\n"]
    pub fn set_elastic_load_balancer(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().elastic_load_balancer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `install_updates_on_boot`.\n"]
    pub fn set_install_updates_on_boot(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().install_updates_on_boot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_shutdown_timeout`.\n"]
    pub fn set_instance_shutdown_timeout(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().instance_shutdown_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `system_packages`.\n"]
    pub fn set_system_packages(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().system_packages = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `use_ebs_optimized_instances`.\n"]
    pub fn set_use_ebs_optimized_instances(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_ebs_optimized_instances = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cloudwatch_configuration`.\n"]
    pub fn set_cloudwatch_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpsworksEcsClusterLayerCloudwatchConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloudwatch_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloudwatch_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ebs_volume`.\n"]
    pub fn set_ebs_volume(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpsworksEcsClusterLayerEbsVolumeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ebs_volume = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ebs_volume = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `load_based_auto_scaling`.\n"]
    pub fn set_load_based_auto_scaling(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpsworksEcsClusterLayerLoadBasedAutoScalingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().load_based_auto_scaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.load_based_auto_scaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_assign_elastic_ips` after provisioning.\n"]
    pub fn auto_assign_elastic_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_assign_elastic_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_assign_public_ips` after provisioning.\n"]
    pub fn auto_assign_public_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_assign_public_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_healing` after provisioning.\n"]
    pub fn auto_healing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_healing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_configure_recipes` after provisioning.\n"]
    pub fn custom_configure_recipes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_configure_recipes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_deploy_recipes` after provisioning.\n"]
    pub fn custom_deploy_recipes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_deploy_recipes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_instance_profile_arn` after provisioning.\n"]
    pub fn custom_instance_profile_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_instance_profile_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_json` after provisioning.\n"]
    pub fn custom_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_security_group_ids` after provisioning.\n"]
    pub fn custom_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.custom_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_setup_recipes` after provisioning.\n"]
    pub fn custom_setup_recipes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_setup_recipes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_shutdown_recipes` after provisioning.\n"]
    pub fn custom_shutdown_recipes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_shutdown_recipes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_undeploy_recipes` after provisioning.\n"]
    pub fn custom_undeploy_recipes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_undeploy_recipes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `drain_elb_on_shutdown` after provisioning.\n"]
    pub fn drain_elb_on_shutdown(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drain_elb_on_shutdown", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ecs_cluster_arn` after provisioning.\n"]
    pub fn ecs_cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ecs_cluster_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elastic_load_balancer` after provisioning.\n"]
    pub fn elastic_load_balancer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.elastic_load_balancer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `install_updates_on_boot` after provisioning.\n"]
    pub fn install_updates_on_boot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.install_updates_on_boot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_shutdown_timeout` after provisioning.\n"]
    pub fn instance_shutdown_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_shutdown_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `system_packages` after provisioning.\n"]
    pub fn system_packages(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.system_packages", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_ebs_optimized_instances` after provisioning.\n"]
    pub fn use_ebs_optimized_instances(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_ebs_optimized_instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_configuration` after provisioning.\n"]
    pub fn cloudwatch_configuration(&self) -> terrars::ListRef<OpsworksEcsClusterLayerCloudwatchConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_based_auto_scaling` after provisioning.\n"]
    pub fn load_based_auto_scaling(&self) -> terrars::ListRef<OpsworksEcsClusterLayerLoadBasedAutoScalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_based_auto_scaling", self.extract_ref()))
    }
}

impl terrars::Referable for OpsworksEcsClusterLayer {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OpsworksEcsClusterLayer { }

impl terrars::ToListMappable for OpsworksEcsClusterLayer {
    type O = terrars::ListRef<OpsworksEcsClusterLayerRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OpsworksEcsClusterLayer_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_opsworks_ecs_cluster_layer".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOpsworksEcsClusterLayer {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub ecs_cluster_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub stack_id: terrars::PrimField<std::string::String>,
}

impl BuildOpsworksEcsClusterLayer {
    pub fn build(self, stack: &mut terrars::Stack) -> OpsworksEcsClusterLayer {
        let out = OpsworksEcsClusterLayer(std::rc::Rc::new(OpsworksEcsClusterLayer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OpsworksEcsClusterLayerData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                auto_assign_elastic_ips: core::default::Default::default(),
                auto_assign_public_ips: core::default::Default::default(),
                auto_healing: core::default::Default::default(),
                custom_configure_recipes: core::default::Default::default(),
                custom_deploy_recipes: core::default::Default::default(),
                custom_instance_profile_arn: core::default::Default::default(),
                custom_json: core::default::Default::default(),
                custom_security_group_ids: core::default::Default::default(),
                custom_setup_recipes: core::default::Default::default(),
                custom_shutdown_recipes: core::default::Default::default(),
                custom_undeploy_recipes: core::default::Default::default(),
                drain_elb_on_shutdown: core::default::Default::default(),
                ecs_cluster_arn: self.ecs_cluster_arn,
                elastic_load_balancer: core::default::Default::default(),
                id: core::default::Default::default(),
                install_updates_on_boot: core::default::Default::default(),
                instance_shutdown_timeout: core::default::Default::default(),
                name: core::default::Default::default(),
                stack_id: self.stack_id,
                system_packages: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                use_ebs_optimized_instances: core::default::Default::default(),
                cloudwatch_configuration: core::default::Default::default(),
                ebs_volume: core::default::Default::default(),
                load_based_auto_scaling: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OpsworksEcsClusterLayerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpsworksEcsClusterLayerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OpsworksEcsClusterLayerRef {
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

    #[doc = "Get a reference to the value of field `auto_assign_elastic_ips` after provisioning.\n"]
    pub fn auto_assign_elastic_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_assign_elastic_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_assign_public_ips` after provisioning.\n"]
    pub fn auto_assign_public_ips(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_assign_public_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_healing` after provisioning.\n"]
    pub fn auto_healing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_healing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_configure_recipes` after provisioning.\n"]
    pub fn custom_configure_recipes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_configure_recipes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_deploy_recipes` after provisioning.\n"]
    pub fn custom_deploy_recipes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_deploy_recipes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_instance_profile_arn` after provisioning.\n"]
    pub fn custom_instance_profile_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_instance_profile_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_json` after provisioning.\n"]
    pub fn custom_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_security_group_ids` after provisioning.\n"]
    pub fn custom_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.custom_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_setup_recipes` after provisioning.\n"]
    pub fn custom_setup_recipes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_setup_recipes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_shutdown_recipes` after provisioning.\n"]
    pub fn custom_shutdown_recipes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_shutdown_recipes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_undeploy_recipes` after provisioning.\n"]
    pub fn custom_undeploy_recipes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_undeploy_recipes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `drain_elb_on_shutdown` after provisioning.\n"]
    pub fn drain_elb_on_shutdown(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.drain_elb_on_shutdown", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ecs_cluster_arn` after provisioning.\n"]
    pub fn ecs_cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ecs_cluster_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elastic_load_balancer` after provisioning.\n"]
    pub fn elastic_load_balancer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.elastic_load_balancer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `install_updates_on_boot` after provisioning.\n"]
    pub fn install_updates_on_boot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.install_updates_on_boot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_shutdown_timeout` after provisioning.\n"]
    pub fn instance_shutdown_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_shutdown_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `system_packages` after provisioning.\n"]
    pub fn system_packages(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.system_packages", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `use_ebs_optimized_instances` after provisioning.\n"]
    pub fn use_ebs_optimized_instances(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_ebs_optimized_instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cloudwatch_configuration` after provisioning.\n"]
    pub fn cloudwatch_configuration(&self) -> terrars::ListRef<OpsworksEcsClusterLayerCloudwatchConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_based_auto_scaling` after provisioning.\n"]
    pub fn load_based_auto_scaling(&self) -> terrars::ListRef<OpsworksEcsClusterLayerLoadBasedAutoScalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_based_auto_scaling", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OpsworksEcsClusterLayerCloudwatchConfigurationElLogStreamsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    buffer_duration: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    datetime_format: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    file: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_fingerprint_lines: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_position: std::option::Option<terrars::PrimField<std::string::String>>,
    log_group_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multiline_start_pattern: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpsworksEcsClusterLayerCloudwatchConfigurationElLogStreamsEl {
    #[doc = "Set the field `batch_count`.\n"]
    pub fn set_batch_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.batch_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `batch_size`.\n"]
    pub fn set_batch_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.batch_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `buffer_duration`.\n"]
    pub fn set_buffer_duration(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.buffer_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `datetime_format`.\n"]
    pub fn set_datetime_format(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.datetime_format = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encoding`.\n"]
    pub fn set_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_fingerprint_lines`.\n"]
    pub fn set_file_fingerprint_lines(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.file_fingerprint_lines = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initial_position`.\n"]
    pub fn set_initial_position(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.initial_position = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `multiline_start_pattern`.\n"]
    pub fn set_multiline_start_pattern(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.multiline_start_pattern = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_zone`.\n"]
    pub fn set_time_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.time_zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpsworksEcsClusterLayerCloudwatchConfigurationElLogStreamsEl {
    type O = terrars::BlockAssignable<OpsworksEcsClusterLayerCloudwatchConfigurationElLogStreamsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksEcsClusterLayerCloudwatchConfigurationElLogStreamsEl {
    #[doc = ""]
    pub file: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub log_group_name: terrars::PrimField<std::string::String>,
}

impl BuildOpsworksEcsClusterLayerCloudwatchConfigurationElLogStreamsEl {
    pub fn build(self) -> OpsworksEcsClusterLayerCloudwatchConfigurationElLogStreamsEl {
        OpsworksEcsClusterLayerCloudwatchConfigurationElLogStreamsEl {
            batch_count: core::default::Default::default(),
            batch_size: core::default::Default::default(),
            buffer_duration: core::default::Default::default(),
            datetime_format: core::default::Default::default(),
            encoding: core::default::Default::default(),
            file: self.file,
            file_fingerprint_lines: core::default::Default::default(),
            initial_position: core::default::Default::default(),
            log_group_name: self.log_group_name,
            multiline_start_pattern: core::default::Default::default(),
            time_zone: core::default::Default::default(),
        }
    }
}

pub struct OpsworksEcsClusterLayerCloudwatchConfigurationElLogStreamsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpsworksEcsClusterLayerCloudwatchConfigurationElLogStreamsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpsworksEcsClusterLayerCloudwatchConfigurationElLogStreamsElRef {
        OpsworksEcsClusterLayerCloudwatchConfigurationElLogStreamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksEcsClusterLayerCloudwatchConfigurationElLogStreamsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `batch_count` after provisioning.\n"]
    pub fn batch_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_count", self.base))
    }

    #[doc = "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
    }

    #[doc = "Get a reference to the value of field `buffer_duration` after provisioning.\n"]
    pub fn buffer_duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.buffer_duration", self.base))
    }

    #[doc = "Get a reference to the value of field `datetime_format` after provisioning.\n"]
    pub fn datetime_format(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.datetime_format", self.base))
    }

    #[doc = "Get a reference to the value of field `encoding` after provisioning.\n"]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc = "Get a reference to the value of field `file_fingerprint_lines` after provisioning.\n"]
    pub fn file_fingerprint_lines(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_fingerprint_lines", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_position` after provisioning.\n"]
    pub fn initial_position(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_position", self.base))
    }

    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `multiline_start_pattern` after provisioning.\n"]
    pub fn multiline_start_pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multiline_start_pattern", self.base))
    }

    #[doc = "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}

#[derive(Serialize, Default)]
struct OpsworksEcsClusterLayerCloudwatchConfigurationElDynamic {
    log_streams: std
    ::option
    ::Option<terrars::DynamicBlock<OpsworksEcsClusterLayerCloudwatchConfigurationElLogStreamsEl>>,
}

#[derive(Serialize)]
pub struct OpsworksEcsClusterLayerCloudwatchConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_streams: std::option::Option<std::vec::Vec<OpsworksEcsClusterLayerCloudwatchConfigurationElLogStreamsEl>>,
    dynamic: OpsworksEcsClusterLayerCloudwatchConfigurationElDynamic,
}

impl OpsworksEcsClusterLayerCloudwatchConfigurationEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_streams`.\n"]
    pub fn set_log_streams(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OpsworksEcsClusterLayerCloudwatchConfigurationElLogStreamsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.log_streams = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.log_streams = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OpsworksEcsClusterLayerCloudwatchConfigurationEl {
    type O = terrars::BlockAssignable<OpsworksEcsClusterLayerCloudwatchConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksEcsClusterLayerCloudwatchConfigurationEl {}

impl BuildOpsworksEcsClusterLayerCloudwatchConfigurationEl {
    pub fn build(self) -> OpsworksEcsClusterLayerCloudwatchConfigurationEl {
        OpsworksEcsClusterLayerCloudwatchConfigurationEl {
            enabled: core::default::Default::default(),
            log_streams: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OpsworksEcsClusterLayerCloudwatchConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpsworksEcsClusterLayerCloudwatchConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpsworksEcsClusterLayerCloudwatchConfigurationElRef {
        OpsworksEcsClusterLayerCloudwatchConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksEcsClusterLayerCloudwatchConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `log_streams` after provisioning.\n"]
    pub fn log_streams(&self) -> terrars::ListRef<OpsworksEcsClusterLayerCloudwatchConfigurationElLogStreamsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_streams", self.base))
    }
}

#[derive(Serialize)]
pub struct OpsworksEcsClusterLayerEbsVolumeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    mount_point: terrars::PrimField<std::string::String>,
    number_of_disks: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    raid_level: std::option::Option<terrars::PrimField<std::string::String>>,
    size: terrars::PrimField<f64>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpsworksEcsClusterLayerEbsVolumeEl {
    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.encrypted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `raid_level`.\n"]
    pub fn set_raid_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.raid_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpsworksEcsClusterLayerEbsVolumeEl {
    type O = terrars::BlockAssignable<OpsworksEcsClusterLayerEbsVolumeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksEcsClusterLayerEbsVolumeEl {
    #[doc = ""]
    pub mount_point: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub number_of_disks: terrars::PrimField<f64>,
    #[doc = ""]
    pub size: terrars::PrimField<f64>,
}

impl BuildOpsworksEcsClusterLayerEbsVolumeEl {
    pub fn build(self) -> OpsworksEcsClusterLayerEbsVolumeEl {
        OpsworksEcsClusterLayerEbsVolumeEl {
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            mount_point: self.mount_point,
            number_of_disks: self.number_of_disks,
            raid_level: core::default::Default::default(),
            size: self.size,
            type_: core::default::Default::default(),
        }
    }
}

pub struct OpsworksEcsClusterLayerEbsVolumeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpsworksEcsClusterLayerEbsVolumeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpsworksEcsClusterLayerEbsVolumeElRef {
        OpsworksEcsClusterLayerEbsVolumeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksEcsClusterLayerEbsVolumeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `mount_point` after provisioning.\n"]
    pub fn mount_point(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_point", self.base))
    }

    #[doc = "Get a reference to the value of field `number_of_disks` after provisioning.\n"]
    pub fn number_of_disks(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_disks", self.base))
    }

    #[doc = "Get a reference to the value of field `raid_level` after provisioning.\n"]
    pub fn raid_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.raid_level", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct OpsworksEcsClusterLayerLoadBasedAutoScalingElDownscalingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alarms: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_metrics_time: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thresholds_wait_time: std::option::Option<terrars::PrimField<f64>>,
}

impl OpsworksEcsClusterLayerLoadBasedAutoScalingElDownscalingEl {
    #[doc = "Set the field `alarms`.\n"]
    pub fn set_alarms(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.alarms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_threshold`.\n"]
    pub fn set_cpu_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_metrics_time`.\n"]
    pub fn set_ignore_metrics_time(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ignore_metrics_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `load_threshold`.\n"]
    pub fn set_load_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.load_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_threshold`.\n"]
    pub fn set_memory_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thresholds_wait_time`.\n"]
    pub fn set_thresholds_wait_time(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.thresholds_wait_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpsworksEcsClusterLayerLoadBasedAutoScalingElDownscalingEl {
    type O = terrars::BlockAssignable<OpsworksEcsClusterLayerLoadBasedAutoScalingElDownscalingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksEcsClusterLayerLoadBasedAutoScalingElDownscalingEl {}

impl BuildOpsworksEcsClusterLayerLoadBasedAutoScalingElDownscalingEl {
    pub fn build(self) -> OpsworksEcsClusterLayerLoadBasedAutoScalingElDownscalingEl {
        OpsworksEcsClusterLayerLoadBasedAutoScalingElDownscalingEl {
            alarms: core::default::Default::default(),
            cpu_threshold: core::default::Default::default(),
            ignore_metrics_time: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            load_threshold: core::default::Default::default(),
            memory_threshold: core::default::Default::default(),
            thresholds_wait_time: core::default::Default::default(),
        }
    }
}

pub struct OpsworksEcsClusterLayerLoadBasedAutoScalingElDownscalingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpsworksEcsClusterLayerLoadBasedAutoScalingElDownscalingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpsworksEcsClusterLayerLoadBasedAutoScalingElDownscalingElRef {
        OpsworksEcsClusterLayerLoadBasedAutoScalingElDownscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksEcsClusterLayerLoadBasedAutoScalingElDownscalingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarms` after provisioning.\n"]
    pub fn alarms(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alarms", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_threshold` after provisioning.\n"]
    pub fn cpu_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_metrics_time` after provisioning.\n"]
    pub fn ignore_metrics_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_metrics_time", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `load_threshold` after provisioning.\n"]
    pub fn load_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_threshold` after provisioning.\n"]
    pub fn memory_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `thresholds_wait_time` after provisioning.\n"]
    pub fn thresholds_wait_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thresholds_wait_time", self.base))
    }
}

#[derive(Serialize)]
pub struct OpsworksEcsClusterLayerLoadBasedAutoScalingElUpscalingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alarms: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_metrics_time: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    thresholds_wait_time: std::option::Option<terrars::PrimField<f64>>,
}

impl OpsworksEcsClusterLayerLoadBasedAutoScalingElUpscalingEl {
    #[doc = "Set the field `alarms`.\n"]
    pub fn set_alarms(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.alarms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_threshold`.\n"]
    pub fn set_cpu_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_metrics_time`.\n"]
    pub fn set_ignore_metrics_time(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ignore_metrics_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `load_threshold`.\n"]
    pub fn set_load_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.load_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_threshold`.\n"]
    pub fn set_memory_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `thresholds_wait_time`.\n"]
    pub fn set_thresholds_wait_time(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.thresholds_wait_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpsworksEcsClusterLayerLoadBasedAutoScalingElUpscalingEl {
    type O = terrars::BlockAssignable<OpsworksEcsClusterLayerLoadBasedAutoScalingElUpscalingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksEcsClusterLayerLoadBasedAutoScalingElUpscalingEl {}

impl BuildOpsworksEcsClusterLayerLoadBasedAutoScalingElUpscalingEl {
    pub fn build(self) -> OpsworksEcsClusterLayerLoadBasedAutoScalingElUpscalingEl {
        OpsworksEcsClusterLayerLoadBasedAutoScalingElUpscalingEl {
            alarms: core::default::Default::default(),
            cpu_threshold: core::default::Default::default(),
            ignore_metrics_time: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            load_threshold: core::default::Default::default(),
            memory_threshold: core::default::Default::default(),
            thresholds_wait_time: core::default::Default::default(),
        }
    }
}

pub struct OpsworksEcsClusterLayerLoadBasedAutoScalingElUpscalingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpsworksEcsClusterLayerLoadBasedAutoScalingElUpscalingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpsworksEcsClusterLayerLoadBasedAutoScalingElUpscalingElRef {
        OpsworksEcsClusterLayerLoadBasedAutoScalingElUpscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksEcsClusterLayerLoadBasedAutoScalingElUpscalingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarms` after provisioning.\n"]
    pub fn alarms(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alarms", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_threshold` after provisioning.\n"]
    pub fn cpu_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_metrics_time` after provisioning.\n"]
    pub fn ignore_metrics_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_metrics_time", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `load_threshold` after provisioning.\n"]
    pub fn load_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_threshold` after provisioning.\n"]
    pub fn memory_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `thresholds_wait_time` after provisioning.\n"]
    pub fn thresholds_wait_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.thresholds_wait_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct OpsworksEcsClusterLayerLoadBasedAutoScalingElDynamic {
    downscaling: std
    ::option
    ::Option<terrars::DynamicBlock<OpsworksEcsClusterLayerLoadBasedAutoScalingElDownscalingEl>>,
    upscaling: std::option::Option<terrars::DynamicBlock<OpsworksEcsClusterLayerLoadBasedAutoScalingElUpscalingEl>>,
}

#[derive(Serialize)]
pub struct OpsworksEcsClusterLayerLoadBasedAutoScalingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    downscaling: std::option::Option<std::vec::Vec<OpsworksEcsClusterLayerLoadBasedAutoScalingElDownscalingEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upscaling: std::option::Option<std::vec::Vec<OpsworksEcsClusterLayerLoadBasedAutoScalingElUpscalingEl>>,
    dynamic: OpsworksEcsClusterLayerLoadBasedAutoScalingElDynamic,
}

impl OpsworksEcsClusterLayerLoadBasedAutoScalingEl {
    #[doc = "Set the field `enable`.\n"]
    pub fn set_enable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `downscaling`.\n"]
    pub fn set_downscaling(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OpsworksEcsClusterLayerLoadBasedAutoScalingElDownscalingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.downscaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.downscaling = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `upscaling`.\n"]
    pub fn set_upscaling(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<OpsworksEcsClusterLayerLoadBasedAutoScalingElUpscalingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.upscaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.upscaling = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for OpsworksEcsClusterLayerLoadBasedAutoScalingEl {
    type O = terrars::BlockAssignable<OpsworksEcsClusterLayerLoadBasedAutoScalingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksEcsClusterLayerLoadBasedAutoScalingEl {}

impl BuildOpsworksEcsClusterLayerLoadBasedAutoScalingEl {
    pub fn build(self) -> OpsworksEcsClusterLayerLoadBasedAutoScalingEl {
        OpsworksEcsClusterLayerLoadBasedAutoScalingEl {
            enable: core::default::Default::default(),
            downscaling: core::default::Default::default(),
            upscaling: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct OpsworksEcsClusterLayerLoadBasedAutoScalingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpsworksEcsClusterLayerLoadBasedAutoScalingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> OpsworksEcsClusterLayerLoadBasedAutoScalingElRef {
        OpsworksEcsClusterLayerLoadBasedAutoScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksEcsClusterLayerLoadBasedAutoScalingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc = "Get a reference to the value of field `downscaling` after provisioning.\n"]
    pub fn downscaling(&self) -> terrars::ListRef<OpsworksEcsClusterLayerLoadBasedAutoScalingElDownscalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.downscaling", self.base))
    }

    #[doc = "Get a reference to the value of field `upscaling` after provisioning.\n"]
    pub fn upscaling(&self) -> terrars::ListRef<OpsworksEcsClusterLayerLoadBasedAutoScalingElUpscalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.upscaling", self.base))
    }
}

#[derive(Serialize, Default)]
struct OpsworksEcsClusterLayerDynamic {
    cloudwatch_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<OpsworksEcsClusterLayerCloudwatchConfigurationEl>>,
    ebs_volume: std::option::Option<terrars::DynamicBlock<OpsworksEcsClusterLayerEbsVolumeEl>>,
    load_based_auto_scaling: std
    ::option
    ::Option<terrars::DynamicBlock<OpsworksEcsClusterLayerLoadBasedAutoScalingEl>>,
}
