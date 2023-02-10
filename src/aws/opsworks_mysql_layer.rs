use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OpsworksMysqlLayerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_assign_elastic_ips: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_assign_public_ips: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_healing: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_configure_recipes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_deploy_recipes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_instance_profile_arn: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_json: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_setup_recipes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_shutdown_recipes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_undeploy_recipes: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drain_elb_on_shutdown: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    elastic_load_balancer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    install_updates_on_boot: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_shutdown_timeout: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_password: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root_password_on_all_instances: Option<PrimField<bool>>,
    stack_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_packages: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    use_ebs_optimized_instances: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cloudwatch_configuration: Option<Vec<OpsworksMysqlLayerCloudwatchConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ebs_volume: Option<Vec<OpsworksMysqlLayerEbsVolumeEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_based_auto_scaling: Option<Vec<OpsworksMysqlLayerLoadBasedAutoScalingEl>>,
    dynamic: OpsworksMysqlLayerDynamic,
}

struct OpsworksMysqlLayer_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<OpsworksMysqlLayerData>,
}

#[derive(Clone)]
pub struct OpsworksMysqlLayer(Rc<OpsworksMysqlLayer_>);

impl OpsworksMysqlLayer {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `auto_assign_elastic_ips`.\n"]
    pub fn set_auto_assign_elastic_ips(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_assign_elastic_ips = Some(v.into());
        self
    }

    #[doc= "Set the field `auto_assign_public_ips`.\n"]
    pub fn set_auto_assign_public_ips(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_assign_public_ips = Some(v.into());
        self
    }

    #[doc= "Set the field `auto_healing`.\n"]
    pub fn set_auto_healing(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_healing = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_configure_recipes`.\n"]
    pub fn set_custom_configure_recipes(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().custom_configure_recipes = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_deploy_recipes`.\n"]
    pub fn set_custom_deploy_recipes(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().custom_deploy_recipes = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_instance_profile_arn`.\n"]
    pub fn set_custom_instance_profile_arn(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_instance_profile_arn = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_json`.\n"]
    pub fn set_custom_json(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().custom_json = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_security_group_ids`.\n"]
    pub fn set_custom_security_group_ids(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().custom_security_group_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_setup_recipes`.\n"]
    pub fn set_custom_setup_recipes(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().custom_setup_recipes = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_shutdown_recipes`.\n"]
    pub fn set_custom_shutdown_recipes(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().custom_shutdown_recipes = Some(v.into());
        self
    }

    #[doc= "Set the field `custom_undeploy_recipes`.\n"]
    pub fn set_custom_undeploy_recipes(self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().custom_undeploy_recipes = Some(v.into());
        self
    }

    #[doc= "Set the field `drain_elb_on_shutdown`.\n"]
    pub fn set_drain_elb_on_shutdown(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().drain_elb_on_shutdown = Some(v.into());
        self
    }

    #[doc= "Set the field `elastic_load_balancer`.\n"]
    pub fn set_elastic_load_balancer(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().elastic_load_balancer = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `install_updates_on_boot`.\n"]
    pub fn set_install_updates_on_boot(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().install_updates_on_boot = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_shutdown_timeout`.\n"]
    pub fn set_instance_shutdown_timeout(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().instance_shutdown_timeout = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `root_password`.\n"]
    pub fn set_root_password(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().root_password = Some(v.into());
        self
    }

    #[doc= "Set the field `root_password_on_all_instances`.\n"]
    pub fn set_root_password_on_all_instances(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().root_password_on_all_instances = Some(v.into());
        self
    }

    #[doc= "Set the field `system_packages`.\n"]
    pub fn set_system_packages(self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().system_packages = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `use_ebs_optimized_instances`.\n"]
    pub fn set_use_ebs_optimized_instances(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().use_ebs_optimized_instances = Some(v.into());
        self
    }

    #[doc= "Set the field `cloudwatch_configuration`.\n"]
    pub fn set_cloudwatch_configuration(
        self,
        v: impl Into<BlockAssignable<OpsworksMysqlLayerCloudwatchConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cloudwatch_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cloudwatch_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `ebs_volume`.\n"]
    pub fn set_ebs_volume(self, v: impl Into<BlockAssignable<OpsworksMysqlLayerEbsVolumeEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ebs_volume = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ebs_volume = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `load_based_auto_scaling`.\n"]
    pub fn set_load_based_auto_scaling(
        self,
        v: impl Into<BlockAssignable<OpsworksMysqlLayerLoadBasedAutoScalingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().load_based_auto_scaling = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.load_based_auto_scaling = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_assign_elastic_ips` after provisioning.\n"]
    pub fn auto_assign_elastic_ips(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_assign_elastic_ips", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_assign_public_ips` after provisioning.\n"]
    pub fn auto_assign_public_ips(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_assign_public_ips", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_healing` after provisioning.\n"]
    pub fn auto_healing(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_healing", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_configure_recipes` after provisioning.\n"]
    pub fn custom_configure_recipes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.custom_configure_recipes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_deploy_recipes` after provisioning.\n"]
    pub fn custom_deploy_recipes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.custom_deploy_recipes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_instance_profile_arn` after provisioning.\n"]
    pub fn custom_instance_profile_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_instance_profile_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_json` after provisioning.\n"]
    pub fn custom_json(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_json", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_security_group_ids` after provisioning.\n"]
    pub fn custom_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.custom_security_group_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_setup_recipes` after provisioning.\n"]
    pub fn custom_setup_recipes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.custom_setup_recipes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_shutdown_recipes` after provisioning.\n"]
    pub fn custom_shutdown_recipes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.custom_shutdown_recipes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_undeploy_recipes` after provisioning.\n"]
    pub fn custom_undeploy_recipes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.custom_undeploy_recipes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `drain_elb_on_shutdown` after provisioning.\n"]
    pub fn drain_elb_on_shutdown(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.drain_elb_on_shutdown", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elastic_load_balancer` after provisioning.\n"]
    pub fn elastic_load_balancer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.elastic_load_balancer", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `install_updates_on_boot` after provisioning.\n"]
    pub fn install_updates_on_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.install_updates_on_boot", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_shutdown_timeout` after provisioning.\n"]
    pub fn instance_shutdown_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_shutdown_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `root_password` after provisioning.\n"]
    pub fn root_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `root_password_on_all_instances` after provisioning.\n"]
    pub fn root_password_on_all_instances(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_password_on_all_instances", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `system_packages` after provisioning.\n"]
    pub fn system_packages(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.system_packages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `use_ebs_optimized_instances` after provisioning.\n"]
    pub fn use_ebs_optimized_instances(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_ebs_optimized_instances", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloudwatch_configuration` after provisioning.\n"]
    pub fn cloudwatch_configuration(&self) -> ListRef<OpsworksMysqlLayerCloudwatchConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloudwatch_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `load_based_auto_scaling` after provisioning.\n"]
    pub fn load_based_auto_scaling(&self) -> ListRef<OpsworksMysqlLayerLoadBasedAutoScalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.load_based_auto_scaling", self.extract_ref()))
    }
}

impl Resource for OpsworksMysqlLayer {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for OpsworksMysqlLayer {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for OpsworksMysqlLayer {
    type O = ListRef<OpsworksMysqlLayerRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(&self))
    }
}

impl Resource_ for OpsworksMysqlLayer_ {
    fn extract_resource_type(&self) -> String {
        "aws_opsworks_mysql_layer".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOpsworksMysqlLayer {
    pub tf_id: String,
    #[doc= ""]
    pub stack_id: PrimField<String>,
}

impl BuildOpsworksMysqlLayer {
    pub fn build(self, stack: &mut Stack) -> OpsworksMysqlLayer {
        let out = OpsworksMysqlLayer(Rc::new(OpsworksMysqlLayer_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(OpsworksMysqlLayerData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                elastic_load_balancer: core::default::Default::default(),
                id: core::default::Default::default(),
                install_updates_on_boot: core::default::Default::default(),
                instance_shutdown_timeout: core::default::Default::default(),
                name: core::default::Default::default(),
                root_password: core::default::Default::default(),
                root_password_on_all_instances: core::default::Default::default(),
                stack_id: self.stack_id,
                system_packages: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                use_ebs_optimized_instances: core::default::Default::default(),
                cloudwatch_configuration: core::default::Default::default(),
                ebs_volume: core::default::Default::default(),
                load_based_auto_scaling: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OpsworksMysqlLayerRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksMysqlLayerRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OpsworksMysqlLayerRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_assign_elastic_ips` after provisioning.\n"]
    pub fn auto_assign_elastic_ips(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_assign_elastic_ips", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_assign_public_ips` after provisioning.\n"]
    pub fn auto_assign_public_ips(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_assign_public_ips", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_healing` after provisioning.\n"]
    pub fn auto_healing(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_healing", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_configure_recipes` after provisioning.\n"]
    pub fn custom_configure_recipes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.custom_configure_recipes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_deploy_recipes` after provisioning.\n"]
    pub fn custom_deploy_recipes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.custom_deploy_recipes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_instance_profile_arn` after provisioning.\n"]
    pub fn custom_instance_profile_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_instance_profile_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_json` after provisioning.\n"]
    pub fn custom_json(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.custom_json", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_security_group_ids` after provisioning.\n"]
    pub fn custom_security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.custom_security_group_ids", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_setup_recipes` after provisioning.\n"]
    pub fn custom_setup_recipes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.custom_setup_recipes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_shutdown_recipes` after provisioning.\n"]
    pub fn custom_shutdown_recipes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.custom_shutdown_recipes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `custom_undeploy_recipes` after provisioning.\n"]
    pub fn custom_undeploy_recipes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.custom_undeploy_recipes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `drain_elb_on_shutdown` after provisioning.\n"]
    pub fn drain_elb_on_shutdown(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.drain_elb_on_shutdown", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `elastic_load_balancer` after provisioning.\n"]
    pub fn elastic_load_balancer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.elastic_load_balancer", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `install_updates_on_boot` after provisioning.\n"]
    pub fn install_updates_on_boot(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.install_updates_on_boot", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_shutdown_timeout` after provisioning.\n"]
    pub fn instance_shutdown_timeout(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_shutdown_timeout", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `root_password` after provisioning.\n"]
    pub fn root_password(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_password", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `root_password_on_all_instances` after provisioning.\n"]
    pub fn root_password_on_all_instances(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.root_password_on_all_instances", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stack_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `system_packages` after provisioning.\n"]
    pub fn system_packages(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.system_packages", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `use_ebs_optimized_instances` after provisioning.\n"]
    pub fn use_ebs_optimized_instances(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.use_ebs_optimized_instances", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cloudwatch_configuration` after provisioning.\n"]
    pub fn cloudwatch_configuration(&self) -> ListRef<OpsworksMysqlLayerCloudwatchConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cloudwatch_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `load_based_auto_scaling` after provisioning.\n"]
    pub fn load_based_auto_scaling(&self) -> ListRef<OpsworksMysqlLayerLoadBasedAutoScalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.load_based_auto_scaling", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OpsworksMysqlLayerCloudwatchConfigurationElLogStreamsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    batch_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buffer_duration: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datetime_format: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encoding: Option<PrimField<String>>,
    file: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_fingerprint_lines: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_position: Option<PrimField<String>>,
    log_group_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiline_start_pattern: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<PrimField<String>>,
}

impl OpsworksMysqlLayerCloudwatchConfigurationElLogStreamsEl {
    #[doc= "Set the field `batch_count`.\n"]
    pub fn set_batch_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_count = Some(v.into());
        self
    }

    #[doc= "Set the field `batch_size`.\n"]
    pub fn set_batch_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.batch_size = Some(v.into());
        self
    }

    #[doc= "Set the field `buffer_duration`.\n"]
    pub fn set_buffer_duration(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.buffer_duration = Some(v.into());
        self
    }

    #[doc= "Set the field `datetime_format`.\n"]
    pub fn set_datetime_format(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.datetime_format = Some(v.into());
        self
    }

    #[doc= "Set the field `encoding`.\n"]
    pub fn set_encoding(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.encoding = Some(v.into());
        self
    }

    #[doc= "Set the field `file_fingerprint_lines`.\n"]
    pub fn set_file_fingerprint_lines(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_fingerprint_lines = Some(v.into());
        self
    }

    #[doc= "Set the field `initial_position`.\n"]
    pub fn set_initial_position(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.initial_position = Some(v.into());
        self
    }

    #[doc= "Set the field `multiline_start_pattern`.\n"]
    pub fn set_multiline_start_pattern(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.multiline_start_pattern = Some(v.into());
        self
    }

    #[doc= "Set the field `time_zone`.\n"]
    pub fn set_time_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.time_zone = Some(v.into());
        self
    }
}

impl ToListMappable for OpsworksMysqlLayerCloudwatchConfigurationElLogStreamsEl {
    type O = BlockAssignable<OpsworksMysqlLayerCloudwatchConfigurationElLogStreamsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksMysqlLayerCloudwatchConfigurationElLogStreamsEl {
    #[doc= ""]
    pub file: PrimField<String>,
    #[doc= ""]
    pub log_group_name: PrimField<String>,
}

impl BuildOpsworksMysqlLayerCloudwatchConfigurationElLogStreamsEl {
    pub fn build(self) -> OpsworksMysqlLayerCloudwatchConfigurationElLogStreamsEl {
        OpsworksMysqlLayerCloudwatchConfigurationElLogStreamsEl {
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

pub struct OpsworksMysqlLayerCloudwatchConfigurationElLogStreamsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksMysqlLayerCloudwatchConfigurationElLogStreamsElRef {
    fn new(shared: StackShared, base: String) -> OpsworksMysqlLayerCloudwatchConfigurationElLogStreamsElRef {
        OpsworksMysqlLayerCloudwatchConfigurationElLogStreamsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksMysqlLayerCloudwatchConfigurationElLogStreamsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `batch_count` after provisioning.\n"]
    pub fn batch_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_count", self.base))
    }

    #[doc= "Get a reference to the value of field `batch_size` after provisioning.\n"]
    pub fn batch_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.batch_size", self.base))
    }

    #[doc= "Get a reference to the value of field `buffer_duration` after provisioning.\n"]
    pub fn buffer_duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.buffer_duration", self.base))
    }

    #[doc= "Get a reference to the value of field `datetime_format` after provisioning.\n"]
    pub fn datetime_format(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.datetime_format", self.base))
    }

    #[doc= "Get a reference to the value of field `encoding` after provisioning.\n"]
    pub fn encoding(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc= "Get a reference to the value of field `file` after provisioning.\n"]
    pub fn file(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file", self.base))
    }

    #[doc= "Get a reference to the value of field `file_fingerprint_lines` after provisioning.\n"]
    pub fn file_fingerprint_lines(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_fingerprint_lines", self.base))
    }

    #[doc= "Get a reference to the value of field `initial_position` after provisioning.\n"]
    pub fn initial_position(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.initial_position", self.base))
    }

    #[doc= "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }

    #[doc= "Get a reference to the value of field `multiline_start_pattern` after provisioning.\n"]
    pub fn multiline_start_pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.multiline_start_pattern", self.base))
    }

    #[doc= "Get a reference to the value of field `time_zone` after provisioning.\n"]
    pub fn time_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.time_zone", self.base))
    }
}

#[derive(Serialize, Default)]
struct OpsworksMysqlLayerCloudwatchConfigurationElDynamic {
    log_streams: Option<DynamicBlock<OpsworksMysqlLayerCloudwatchConfigurationElLogStreamsEl>>,
}

#[derive(Serialize)]
pub struct OpsworksMysqlLayerCloudwatchConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_streams: Option<Vec<OpsworksMysqlLayerCloudwatchConfigurationElLogStreamsEl>>,
    dynamic: OpsworksMysqlLayerCloudwatchConfigurationElDynamic,
}

impl OpsworksMysqlLayerCloudwatchConfigurationEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `log_streams`.\n"]
    pub fn set_log_streams(
        mut self,
        v: impl Into<BlockAssignable<OpsworksMysqlLayerCloudwatchConfigurationElLogStreamsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.log_streams = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.log_streams = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for OpsworksMysqlLayerCloudwatchConfigurationEl {
    type O = BlockAssignable<OpsworksMysqlLayerCloudwatchConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksMysqlLayerCloudwatchConfigurationEl {}

impl BuildOpsworksMysqlLayerCloudwatchConfigurationEl {
    pub fn build(self) -> OpsworksMysqlLayerCloudwatchConfigurationEl {
        OpsworksMysqlLayerCloudwatchConfigurationEl {
            enabled: core::default::Default::default(),
            log_streams: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct OpsworksMysqlLayerCloudwatchConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksMysqlLayerCloudwatchConfigurationElRef {
    fn new(shared: StackShared, base: String) -> OpsworksMysqlLayerCloudwatchConfigurationElRef {
        OpsworksMysqlLayerCloudwatchConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksMysqlLayerCloudwatchConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `log_streams` after provisioning.\n"]
    pub fn log_streams(&self) -> ListRef<OpsworksMysqlLayerCloudwatchConfigurationElLogStreamsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.log_streams", self.base))
    }
}

#[derive(Serialize)]
pub struct OpsworksMysqlLayerEbsVolumeEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    encrypted: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    iops: Option<PrimField<f64>>,
    mount_point: PrimField<String>,
    number_of_disks: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    raid_level: Option<PrimField<String>>,
    size: PrimField<f64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl OpsworksMysqlLayerEbsVolumeEl {
    #[doc= "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.encrypted = Some(v.into());
        self
    }

    #[doc= "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.iops = Some(v.into());
        self
    }

    #[doc= "Set the field `raid_level`.\n"]
    pub fn set_raid_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.raid_level = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for OpsworksMysqlLayerEbsVolumeEl {
    type O = BlockAssignable<OpsworksMysqlLayerEbsVolumeEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksMysqlLayerEbsVolumeEl {
    #[doc= ""]
    pub mount_point: PrimField<String>,
    #[doc= ""]
    pub number_of_disks: PrimField<f64>,
    #[doc= ""]
    pub size: PrimField<f64>,
}

impl BuildOpsworksMysqlLayerEbsVolumeEl {
    pub fn build(self) -> OpsworksMysqlLayerEbsVolumeEl {
        OpsworksMysqlLayerEbsVolumeEl {
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

pub struct OpsworksMysqlLayerEbsVolumeElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksMysqlLayerEbsVolumeElRef {
    fn new(shared: StackShared, base: String) -> OpsworksMysqlLayerEbsVolumeElRef {
        OpsworksMysqlLayerEbsVolumeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksMysqlLayerEbsVolumeElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }

    #[doc= "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc= "Get a reference to the value of field `mount_point` after provisioning.\n"]
    pub fn mount_point(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mount_point", self.base))
    }

    #[doc= "Get a reference to the value of field `number_of_disks` after provisioning.\n"]
    pub fn number_of_disks(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.number_of_disks", self.base))
    }

    #[doc= "Get a reference to the value of field `raid_level` after provisioning.\n"]
    pub fn raid_level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.raid_level", self.base))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct OpsworksMysqlLayerLoadBasedAutoScalingElDownscalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alarms: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_metrics_time: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thresholds_wait_time: Option<PrimField<f64>>,
}

impl OpsworksMysqlLayerLoadBasedAutoScalingElDownscalingEl {
    #[doc= "Set the field `alarms`.\n"]
    pub fn set_alarms(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.alarms = Some(v.into());
        self
    }

    #[doc= "Set the field `cpu_threshold`.\n"]
    pub fn set_cpu_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `ignore_metrics_time`.\n"]
    pub fn set_ignore_metrics_time(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ignore_metrics_time = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.instance_count = Some(v.into());
        self
    }

    #[doc= "Set the field `load_threshold`.\n"]
    pub fn set_load_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.load_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `memory_threshold`.\n"]
    pub fn set_memory_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `thresholds_wait_time`.\n"]
    pub fn set_thresholds_wait_time(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.thresholds_wait_time = Some(v.into());
        self
    }
}

impl ToListMappable for OpsworksMysqlLayerLoadBasedAutoScalingElDownscalingEl {
    type O = BlockAssignable<OpsworksMysqlLayerLoadBasedAutoScalingElDownscalingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksMysqlLayerLoadBasedAutoScalingElDownscalingEl {}

impl BuildOpsworksMysqlLayerLoadBasedAutoScalingElDownscalingEl {
    pub fn build(self) -> OpsworksMysqlLayerLoadBasedAutoScalingElDownscalingEl {
        OpsworksMysqlLayerLoadBasedAutoScalingElDownscalingEl {
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

pub struct OpsworksMysqlLayerLoadBasedAutoScalingElDownscalingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksMysqlLayerLoadBasedAutoScalingElDownscalingElRef {
    fn new(shared: StackShared, base: String) -> OpsworksMysqlLayerLoadBasedAutoScalingElDownscalingElRef {
        OpsworksMysqlLayerLoadBasedAutoScalingElDownscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksMysqlLayerLoadBasedAutoScalingElDownscalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `alarms` after provisioning.\n"]
    pub fn alarms(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.alarms", self.base))
    }

    #[doc= "Get a reference to the value of field `cpu_threshold` after provisioning.\n"]
    pub fn cpu_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_threshold", self.base))
    }

    #[doc= "Get a reference to the value of field `ignore_metrics_time` after provisioning.\n"]
    pub fn ignore_metrics_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_metrics_time", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc= "Get a reference to the value of field `load_threshold` after provisioning.\n"]
    pub fn load_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_threshold", self.base))
    }

    #[doc= "Get a reference to the value of field `memory_threshold` after provisioning.\n"]
    pub fn memory_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_threshold", self.base))
    }

    #[doc= "Get a reference to the value of field `thresholds_wait_time` after provisioning.\n"]
    pub fn thresholds_wait_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.thresholds_wait_time", self.base))
    }
}

#[derive(Serialize)]
pub struct OpsworksMysqlLayerLoadBasedAutoScalingElUpscalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    alarms: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cpu_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ignore_metrics_time: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    load_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_threshold: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thresholds_wait_time: Option<PrimField<f64>>,
}

impl OpsworksMysqlLayerLoadBasedAutoScalingElUpscalingEl {
    #[doc= "Set the field `alarms`.\n"]
    pub fn set_alarms(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.alarms = Some(v.into());
        self
    }

    #[doc= "Set the field `cpu_threshold`.\n"]
    pub fn set_cpu_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.cpu_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `ignore_metrics_time`.\n"]
    pub fn set_ignore_metrics_time(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.ignore_metrics_time = Some(v.into());
        self
    }

    #[doc= "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.instance_count = Some(v.into());
        self
    }

    #[doc= "Set the field `load_threshold`.\n"]
    pub fn set_load_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.load_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `memory_threshold`.\n"]
    pub fn set_memory_threshold(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_threshold = Some(v.into());
        self
    }

    #[doc= "Set the field `thresholds_wait_time`.\n"]
    pub fn set_thresholds_wait_time(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.thresholds_wait_time = Some(v.into());
        self
    }
}

impl ToListMappable for OpsworksMysqlLayerLoadBasedAutoScalingElUpscalingEl {
    type O = BlockAssignable<OpsworksMysqlLayerLoadBasedAutoScalingElUpscalingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksMysqlLayerLoadBasedAutoScalingElUpscalingEl {}

impl BuildOpsworksMysqlLayerLoadBasedAutoScalingElUpscalingEl {
    pub fn build(self) -> OpsworksMysqlLayerLoadBasedAutoScalingElUpscalingEl {
        OpsworksMysqlLayerLoadBasedAutoScalingElUpscalingEl {
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

pub struct OpsworksMysqlLayerLoadBasedAutoScalingElUpscalingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksMysqlLayerLoadBasedAutoScalingElUpscalingElRef {
    fn new(shared: StackShared, base: String) -> OpsworksMysqlLayerLoadBasedAutoScalingElUpscalingElRef {
        OpsworksMysqlLayerLoadBasedAutoScalingElUpscalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksMysqlLayerLoadBasedAutoScalingElUpscalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `alarms` after provisioning.\n"]
    pub fn alarms(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.alarms", self.base))
    }

    #[doc= "Get a reference to the value of field `cpu_threshold` after provisioning.\n"]
    pub fn cpu_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu_threshold", self.base))
    }

    #[doc= "Get a reference to the value of field `ignore_metrics_time` after provisioning.\n"]
    pub fn ignore_metrics_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ignore_metrics_time", self.base))
    }

    #[doc= "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc= "Get a reference to the value of field `load_threshold` after provisioning.\n"]
    pub fn load_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.load_threshold", self.base))
    }

    #[doc= "Get a reference to the value of field `memory_threshold` after provisioning.\n"]
    pub fn memory_threshold(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_threshold", self.base))
    }

    #[doc= "Get a reference to the value of field `thresholds_wait_time` after provisioning.\n"]
    pub fn thresholds_wait_time(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.thresholds_wait_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct OpsworksMysqlLayerLoadBasedAutoScalingElDynamic {
    downscaling: Option<DynamicBlock<OpsworksMysqlLayerLoadBasedAutoScalingElDownscalingEl>>,
    upscaling: Option<DynamicBlock<OpsworksMysqlLayerLoadBasedAutoScalingElUpscalingEl>>,
}

#[derive(Serialize)]
pub struct OpsworksMysqlLayerLoadBasedAutoScalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enable: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    downscaling: Option<Vec<OpsworksMysqlLayerLoadBasedAutoScalingElDownscalingEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upscaling: Option<Vec<OpsworksMysqlLayerLoadBasedAutoScalingElUpscalingEl>>,
    dynamic: OpsworksMysqlLayerLoadBasedAutoScalingElDynamic,
}

impl OpsworksMysqlLayerLoadBasedAutoScalingEl {
    #[doc= "Set the field `enable`.\n"]
    pub fn set_enable(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enable = Some(v.into());
        self
    }

    #[doc= "Set the field `downscaling`.\n"]
    pub fn set_downscaling(
        mut self,
        v: impl Into<BlockAssignable<OpsworksMysqlLayerLoadBasedAutoScalingElDownscalingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.downscaling = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.downscaling = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `upscaling`.\n"]
    pub fn set_upscaling(
        mut self,
        v: impl Into<BlockAssignable<OpsworksMysqlLayerLoadBasedAutoScalingElUpscalingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.upscaling = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.upscaling = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for OpsworksMysqlLayerLoadBasedAutoScalingEl {
    type O = BlockAssignable<OpsworksMysqlLayerLoadBasedAutoScalingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksMysqlLayerLoadBasedAutoScalingEl {}

impl BuildOpsworksMysqlLayerLoadBasedAutoScalingEl {
    pub fn build(self) -> OpsworksMysqlLayerLoadBasedAutoScalingEl {
        OpsworksMysqlLayerLoadBasedAutoScalingEl {
            enable: core::default::Default::default(),
            downscaling: core::default::Default::default(),
            upscaling: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct OpsworksMysqlLayerLoadBasedAutoScalingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for OpsworksMysqlLayerLoadBasedAutoScalingElRef {
    fn new(shared: StackShared, base: String) -> OpsworksMysqlLayerLoadBasedAutoScalingElRef {
        OpsworksMysqlLayerLoadBasedAutoScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksMysqlLayerLoadBasedAutoScalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc= "Get a reference to the value of field `downscaling` after provisioning.\n"]
    pub fn downscaling(&self) -> ListRef<OpsworksMysqlLayerLoadBasedAutoScalingElDownscalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.downscaling", self.base))
    }

    #[doc= "Get a reference to the value of field `upscaling` after provisioning.\n"]
    pub fn upscaling(&self) -> ListRef<OpsworksMysqlLayerLoadBasedAutoScalingElUpscalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.upscaling", self.base))
    }
}

#[derive(Serialize, Default)]
struct OpsworksMysqlLayerDynamic {
    cloudwatch_configuration: Option<DynamicBlock<OpsworksMysqlLayerCloudwatchConfigurationEl>>,
    ebs_volume: Option<DynamicBlock<OpsworksMysqlLayerEbsVolumeEl>>,
    load_based_auto_scaling: Option<DynamicBlock<OpsworksMysqlLayerLoadBasedAutoScalingEl>>,
}
