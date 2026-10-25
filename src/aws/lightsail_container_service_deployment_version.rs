#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LightsailContainerServiceDeploymentVersionData_ {
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
    service_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container: std::option::Option<std::vec::Vec<LightsailContainerServiceDeploymentVersion_Container>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_endpoint: std::option::Option<std::vec::Vec<LightsailContainerServiceDeploymentVersion_PublicEndpoint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LightsailContainerServiceDeploymentVersion_Timeouts>,
    dynamic: LightsailContainerServiceDeploymentVersionDynamic,
}

struct LightsailContainerServiceDeploymentVersion_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LightsailContainerServiceDeploymentVersionData_>,
}

#[derive(Clone)]
pub struct LightsailContainerServiceDeploymentVersion(std::rc::Rc<LightsailContainerServiceDeploymentVersion_>);

impl LightsailContainerServiceDeploymentVersion {
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

    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LightsailContainerServiceDeploymentVersion_Container>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().container = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.container = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `public_endpoint`.\n"]
    pub fn set_public_endpoint(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LightsailContainerServiceDeploymentVersion_PublicEndpoint>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().public_endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.public_endpoint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LightsailContainerServiceDeploymentVersion_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_endpoint` after provisioning.\n"]
    pub fn public_endpoint(&self) -> terrars::ListRef<LightsailContainerServiceDeploymentVersion_PublicEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LightsailContainerServiceDeploymentVersion_TimeoutsRef {
        LightsailContainerServiceDeploymentVersion_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for LightsailContainerServiceDeploymentVersion {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LightsailContainerServiceDeploymentVersion { }

impl terrars::ToListMappable for LightsailContainerServiceDeploymentVersion {
    type O = terrars::ListRef<LightsailContainerServiceDeploymentVersionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LightsailContainerServiceDeploymentVersion_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lightsail_container_service_deployment_version".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLightsailContainerServiceDeploymentVersion {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub service_name: terrars::PrimField<std::string::String>,
}

impl BuildLightsailContainerServiceDeploymentVersion {
    pub fn build(self, stack: &mut terrars::Stack) -> LightsailContainerServiceDeploymentVersion {
        let out =
            LightsailContainerServiceDeploymentVersion(std::rc::Rc::new(LightsailContainerServiceDeploymentVersion_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(LightsailContainerServiceDeploymentVersionData_ {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    provisioner: core::default::Default::default(),
                    id: core::default::Default::default(),
                    region: core::default::Default::default(),
                    service_name: self.service_name,
                    container: core::default::Default::default(),
                    public_endpoint: core::default::Default::default(),
                    timeouts: core::default::Default::default(),
                    dynamic: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LightsailContainerServiceDeploymentVersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailContainerServiceDeploymentVersionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LightsailContainerServiceDeploymentVersionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_endpoint` after provisioning.\n"]
    pub fn public_endpoint(&self) -> terrars::ListRef<LightsailContainerServiceDeploymentVersion_PublicEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LightsailContainerServiceDeploymentVersion_TimeoutsRef {
        LightsailContainerServiceDeploymentVersion_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct LightsailContainerServiceDeploymentVersionDynamic {
    container: std::option::Option<terrars::DynamicBlock<LightsailContainerServiceDeploymentVersion_Container>>,
    public_endpoint: std
    ::option
    ::Option<terrars::DynamicBlock<LightsailContainerServiceDeploymentVersion_PublicEndpoint>>,
}

#[derive(Serialize)]
pub struct LightsailContainerServiceDeploymentVersion_Container {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    container_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ports: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl LightsailContainerServiceDeploymentVersion_Container {
    #[doc = "Set the field `command`.\n"]
    pub fn set_command(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ports`.\n"]
    pub fn set_ports(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ports = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LightsailContainerServiceDeploymentVersion_Container {
    type O = terrars::BlockAssignable<LightsailContainerServiceDeploymentVersion_Container>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailContainerServiceDeploymentVersion_Container {
    #[doc = ""]
    pub container_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub image: terrars::PrimField<std::string::String>,
}

impl BuildLightsailContainerServiceDeploymentVersion_Container {
    pub fn build(self) -> LightsailContainerServiceDeploymentVersion_Container {
        LightsailContainerServiceDeploymentVersion_Container {
            command: core::default::Default::default(),
            container_name: self.container_name,
            environment: core::default::Default::default(),
            image: self.image,
            ports: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LightsailContainerServiceDeploymentVersion_ContainerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailContainerServiceDeploymentVersion_ContainerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LightsailContainerServiceDeploymentVersion_ContainerRef {
        LightsailContainerServiceDeploymentVersion_ContainerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailContainerServiceDeploymentVersion_ContainerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc = "Get a reference to the value of field `container_name` after provisioning.\n"]
    pub fn container_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_name", self.base))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `ports` after provisioning.\n"]
    pub fn ports(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.ports", self.base))
    }
}

#[derive(Serialize)]
pub struct LightsailContainerServiceDeploymentVersion_PublicEndpoint_HealthCheck {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    healthy_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    success_codes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unhealthy_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl LightsailContainerServiceDeploymentVersion_PublicEndpoint_HealthCheck {
    #[doc = "Set the field `healthy_threshold`.\n"]
    pub fn set_healthy_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.healthy_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval_seconds`.\n"]
    pub fn set_interval_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `success_codes`.\n"]
    pub fn set_success_codes(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.success_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_seconds`.\n"]
    pub fn set_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unhealthy_threshold`.\n"]
    pub fn set_unhealthy_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.unhealthy_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LightsailContainerServiceDeploymentVersion_PublicEndpoint_HealthCheck {
    type O = terrars::BlockAssignable<LightsailContainerServiceDeploymentVersion_PublicEndpoint_HealthCheck>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailContainerServiceDeploymentVersion_PublicEndpoint_HealthCheck {}

impl BuildLightsailContainerServiceDeploymentVersion_PublicEndpoint_HealthCheck {
    pub fn build(self) -> LightsailContainerServiceDeploymentVersion_PublicEndpoint_HealthCheck {
        LightsailContainerServiceDeploymentVersion_PublicEndpoint_HealthCheck {
            healthy_threshold: core::default::Default::default(),
            interval_seconds: core::default::Default::default(),
            path: core::default::Default::default(),
            success_codes: core::default::Default::default(),
            timeout_seconds: core::default::Default::default(),
            unhealthy_threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LightsailContainerServiceDeploymentVersion_PublicEndpoint_HealthCheckRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailContainerServiceDeploymentVersion_PublicEndpoint_HealthCheckRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LightsailContainerServiceDeploymentVersion_PublicEndpoint_HealthCheckRef {
        LightsailContainerServiceDeploymentVersion_PublicEndpoint_HealthCheckRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailContainerServiceDeploymentVersion_PublicEndpoint_HealthCheckRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `healthy_threshold` after provisioning.\n"]
    pub fn healthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.healthy_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_seconds` after provisioning.\n"]
    pub fn interval_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `success_codes` after provisioning.\n"]
    pub fn success_codes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.success_codes", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_seconds` after provisioning.\n"]
    pub fn timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `unhealthy_threshold` after provisioning.\n"]
    pub fn unhealthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unhealthy_threshold", self.base))
    }
}

#[derive(Serialize, Default)]
struct LightsailContainerServiceDeploymentVersion_PublicEndpointDynamic {
    health_check: std
    ::option
    ::Option<terrars::DynamicBlock<LightsailContainerServiceDeploymentVersion_PublicEndpoint_HealthCheck>>,
}

#[derive(Serialize)]
pub struct LightsailContainerServiceDeploymentVersion_PublicEndpoint {
    container_name: terrars::PrimField<std::string::String>,
    container_port: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check: std
    ::option
    ::Option<std::vec::Vec<LightsailContainerServiceDeploymentVersion_PublicEndpoint_HealthCheck>>,
    dynamic: LightsailContainerServiceDeploymentVersion_PublicEndpointDynamic,
}

impl LightsailContainerServiceDeploymentVersion_PublicEndpoint {
    #[doc = "Set the field `health_check`.\n"]
    pub fn set_health_check(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<LightsailContainerServiceDeploymentVersion_PublicEndpoint_HealthCheck>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.health_check = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.health_check = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LightsailContainerServiceDeploymentVersion_PublicEndpoint {
    type O = terrars::BlockAssignable<LightsailContainerServiceDeploymentVersion_PublicEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailContainerServiceDeploymentVersion_PublicEndpoint {
    #[doc = ""]
    pub container_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub container_port: terrars::PrimField<f64>,
}

impl BuildLightsailContainerServiceDeploymentVersion_PublicEndpoint {
    pub fn build(self) -> LightsailContainerServiceDeploymentVersion_PublicEndpoint {
        LightsailContainerServiceDeploymentVersion_PublicEndpoint {
            container_name: self.container_name,
            container_port: self.container_port,
            health_check: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LightsailContainerServiceDeploymentVersion_PublicEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailContainerServiceDeploymentVersion_PublicEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LightsailContainerServiceDeploymentVersion_PublicEndpointRef {
        LightsailContainerServiceDeploymentVersion_PublicEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailContainerServiceDeploymentVersion_PublicEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_name` after provisioning.\n"]
    pub fn container_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_name", self.base))
    }

    #[doc = "Get a reference to the value of field `container_port` after provisioning.\n"]
    pub fn container_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_port", self.base))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(
        &self,
    ) -> terrars::ListRef<LightsailContainerServiceDeploymentVersion_PublicEndpoint_HealthCheckRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check", self.base))
    }
}

#[derive(Serialize)]
pub struct LightsailContainerServiceDeploymentVersion_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LightsailContainerServiceDeploymentVersion_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LightsailContainerServiceDeploymentVersion_Timeouts {
    type O = terrars::BlockAssignable<LightsailContainerServiceDeploymentVersion_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLightsailContainerServiceDeploymentVersion_Timeouts {}

impl BuildLightsailContainerServiceDeploymentVersion_Timeouts {
    pub fn build(self) -> LightsailContainerServiceDeploymentVersion_Timeouts {
        LightsailContainerServiceDeploymentVersion_Timeouts { create: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct LightsailContainerServiceDeploymentVersion_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LightsailContainerServiceDeploymentVersion_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LightsailContainerServiceDeploymentVersion_TimeoutsRef {
        LightsailContainerServiceDeploymentVersion_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LightsailContainerServiceDeploymentVersion_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
