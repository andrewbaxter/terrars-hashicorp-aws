#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VerifiedaccessEndpointData_ {
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
    application_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    attachment_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_domain_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    endpoint_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_document: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    verified_access_group_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_options: std::option::Option<std::vec::Vec<VerifiedaccessEndpoint_CidrOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_options: std::option::Option<std::vec::Vec<VerifiedaccessEndpoint_LoadBalancerOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_options: std::option::Option<std::vec::Vec<VerifiedaccessEndpoint_NetworkInterfaceOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rds_options: std::option::Option<std::vec::Vec<VerifiedaccessEndpoint_RdsOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sse_specification: std::option::Option<std::vec::Vec<VerifiedaccessEndpoint_SseSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VerifiedaccessEndpoint_Timeouts>,
    dynamic: VerifiedaccessEndpointDynamic,
}

struct VerifiedaccessEndpoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VerifiedaccessEndpointData_>,
}

#[derive(Clone)]
pub struct VerifiedaccessEndpoint(std::rc::Rc<VerifiedaccessEndpoint_>);

impl VerifiedaccessEndpoint {
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

    #[doc = "Set the field `application_domain`.\n"]
    pub fn set_application_domain(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().application_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain_certificate_arn`.\n"]
    pub fn set_domain_certificate_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().domain_certificate_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint_domain_prefix`.\n"]
    pub fn set_endpoint_domain_prefix(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().endpoint_domain_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_document`.\n"]
    pub fn set_policy_document(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().policy_document = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().security_group_ids = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `cidr_options`.\n"]
    pub fn set_cidr_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VerifiedaccessEndpoint_CidrOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cidr_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cidr_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `load_balancer_options`.\n"]
    pub fn set_load_balancer_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VerifiedaccessEndpoint_LoadBalancerOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().load_balancer_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.load_balancer_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_interface_options`.\n"]
    pub fn set_network_interface_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VerifiedaccessEndpoint_NetworkInterfaceOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_interface_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_interface_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rds_options`.\n"]
    pub fn set_rds_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VerifiedaccessEndpoint_RdsOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rds_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rds_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sse_specification`.\n"]
    pub fn set_sse_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VerifiedaccessEndpoint_SseSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sse_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sse_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VerifiedaccessEndpoint_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `application_domain` after provisioning.\n"]
    pub fn application_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attachment_type` after provisioning.\n"]
    pub fn attachment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attachment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_validation_domain` after provisioning.\n"]
    pub fn device_validation_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_validation_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_certificate_arn` after provisioning.\n"]
    pub fn domain_certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_domain` after provisioning.\n"]
    pub fn endpoint_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_domain_prefix` after provisioning.\n"]
    pub fn endpoint_domain_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_domain_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_document` after provisioning.\n"]
    pub fn policy_document(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_document", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verified_access_group_id` after provisioning.\n"]
    pub fn verified_access_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verified_access_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verified_access_instance_id` after provisioning.\n"]
    pub fn verified_access_instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verified_access_instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_options` after provisioning.\n"]
    pub fn cidr_options(&self) -> terrars::ListRef<VerifiedaccessEndpoint_CidrOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cidr_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer_options` after provisioning.\n"]
    pub fn load_balancer_options(&self) -> terrars::ListRef<VerifiedaccessEndpoint_LoadBalancerOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_options` after provisioning.\n"]
    pub fn network_interface_options(&self) -> terrars::ListRef<VerifiedaccessEndpoint_NetworkInterfaceOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rds_options` after provisioning.\n"]
    pub fn rds_options(&self) -> terrars::ListRef<VerifiedaccessEndpoint_RdsOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rds_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sse_specification` after provisioning.\n"]
    pub fn sse_specification(&self) -> terrars::ListRef<VerifiedaccessEndpoint_SseSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sse_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VerifiedaccessEndpoint_TimeoutsRef {
        VerifiedaccessEndpoint_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for VerifiedaccessEndpoint {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VerifiedaccessEndpoint { }

impl terrars::ToListMappable for VerifiedaccessEndpoint {
    type O = terrars::ListRef<VerifiedaccessEndpointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VerifiedaccessEndpoint_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_verifiedaccess_endpoint".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVerifiedaccessEndpoint {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub attachment_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub endpoint_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub verified_access_group_id: terrars::PrimField<std::string::String>,
}

impl BuildVerifiedaccessEndpoint {
    pub fn build(self, stack: &mut terrars::Stack) -> VerifiedaccessEndpoint {
        let out = VerifiedaccessEndpoint(std::rc::Rc::new(VerifiedaccessEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VerifiedaccessEndpointData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                application_domain: core::default::Default::default(),
                attachment_type: self.attachment_type,
                description: core::default::Default::default(),
                domain_certificate_arn: core::default::Default::default(),
                endpoint_domain_prefix: core::default::Default::default(),
                endpoint_type: self.endpoint_type,
                id: core::default::Default::default(),
                policy_document: core::default::Default::default(),
                region: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                verified_access_group_id: self.verified_access_group_id,
                cidr_options: core::default::Default::default(),
                load_balancer_options: core::default::Default::default(),
                network_interface_options: core::default::Default::default(),
                rds_options: core::default::Default::default(),
                sse_specification: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VerifiedaccessEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedaccessEndpointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VerifiedaccessEndpointRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_domain` after provisioning.\n"]
    pub fn application_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attachment_type` after provisioning.\n"]
    pub fn attachment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attachment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_validation_domain` after provisioning.\n"]
    pub fn device_validation_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_validation_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_certificate_arn` after provisioning.\n"]
    pub fn domain_certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_certificate_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_domain` after provisioning.\n"]
    pub fn endpoint_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_domain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_domain_prefix` after provisioning.\n"]
    pub fn endpoint_domain_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_domain_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_type` after provisioning.\n"]
    pub fn endpoint_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_document` after provisioning.\n"]
    pub fn policy_document(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_document", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verified_access_group_id` after provisioning.\n"]
    pub fn verified_access_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verified_access_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verified_access_instance_id` after provisioning.\n"]
    pub fn verified_access_instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verified_access_instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_options` after provisioning.\n"]
    pub fn cidr_options(&self) -> terrars::ListRef<VerifiedaccessEndpoint_CidrOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cidr_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer_options` after provisioning.\n"]
    pub fn load_balancer_options(&self) -> terrars::ListRef<VerifiedaccessEndpoint_LoadBalancerOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_options` after provisioning.\n"]
    pub fn network_interface_options(&self) -> terrars::ListRef<VerifiedaccessEndpoint_NetworkInterfaceOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rds_options` after provisioning.\n"]
    pub fn rds_options(&self) -> terrars::ListRef<VerifiedaccessEndpoint_RdsOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rds_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sse_specification` after provisioning.\n"]
    pub fn sse_specification(&self) -> terrars::ListRef<VerifiedaccessEndpoint_SseSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sse_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VerifiedaccessEndpoint_TimeoutsRef {
        VerifiedaccessEndpoint_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VerifiedaccessEndpointDynamic {
    cidr_options: std::option::Option<terrars::DynamicBlock<VerifiedaccessEndpoint_CidrOptions>>,
    load_balancer_options: std::option::Option<terrars::DynamicBlock<VerifiedaccessEndpoint_LoadBalancerOptions>>,
    network_interface_options: std
    ::option
    ::Option<terrars::DynamicBlock<VerifiedaccessEndpoint_NetworkInterfaceOptions>>,
    rds_options: std::option::Option<terrars::DynamicBlock<VerifiedaccessEndpoint_RdsOptions>>,
    sse_specification: std::option::Option<terrars::DynamicBlock<VerifiedaccessEndpoint_SseSpecification>>,
}

#[derive(Serialize)]
pub struct VerifiedaccessEndpoint_CidrOptions_PortRange {
    from_port: terrars::PrimField<f64>,
    to_port: terrars::PrimField<f64>,
}

impl VerifiedaccessEndpoint_CidrOptions_PortRange { }

impl terrars::ToListMappable for VerifiedaccessEndpoint_CidrOptions_PortRange {
    type O = terrars::BlockAssignable<VerifiedaccessEndpoint_CidrOptions_PortRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessEndpoint_CidrOptions_PortRange {
    #[doc = ""]
    pub from_port: terrars::PrimField<f64>,
    #[doc = ""]
    pub to_port: terrars::PrimField<f64>,
}

impl BuildVerifiedaccessEndpoint_CidrOptions_PortRange {
    pub fn build(self) -> VerifiedaccessEndpoint_CidrOptions_PortRange {
        VerifiedaccessEndpoint_CidrOptions_PortRange {
            from_port: self.from_port,
            to_port: self.to_port,
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedaccessEndpoint_CidrOptions_PortRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedaccessEndpoint_CidrOptions_PortRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedaccessEndpoint_CidrOptions_PortRangeRef {
        VerifiedaccessEndpoint_CidrOptions_PortRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessEndpoint_CidrOptions_PortRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize, Default)]
struct VerifiedaccessEndpoint_CidrOptionsDynamic {
    port_range: std::option::Option<terrars::DynamicBlock<VerifiedaccessEndpoint_CidrOptions_PortRange>>,
}

#[derive(Serialize)]
pub struct VerifiedaccessEndpoint_CidrOptions {
    cidr: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std::option::Option<std::vec::Vec<VerifiedaccessEndpoint_CidrOptions_PortRange>>,
    dynamic: VerifiedaccessEndpoint_CidrOptionsDynamic,
}

impl VerifiedaccessEndpoint_CidrOptions {
    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnet_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VerifiedaccessEndpoint_CidrOptions_PortRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.port_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.port_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VerifiedaccessEndpoint_CidrOptions {
    type O = terrars::BlockAssignable<VerifiedaccessEndpoint_CidrOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessEndpoint_CidrOptions {
    #[doc = ""]
    pub cidr: terrars::PrimField<std::string::String>,
}

impl BuildVerifiedaccessEndpoint_CidrOptions {
    pub fn build(self) -> VerifiedaccessEndpoint_CidrOptions {
        VerifiedaccessEndpoint_CidrOptions {
            cidr: self.cidr,
            protocol: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            port_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedaccessEndpoint_CidrOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedaccessEndpoint_CidrOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VerifiedaccessEndpoint_CidrOptionsRef {
        VerifiedaccessEndpoint_CidrOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessEndpoint_CidrOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}

#[derive(Serialize, Default)]
struct VerifiedaccessEndpoint_LoadBalancerOptionsDynamic {
    port_range: std::option::Option<terrars::DynamicBlock<VerifiedaccessEndpoint_CidrOptions_PortRange>>,
}

#[derive(Serialize)]
pub struct VerifiedaccessEndpoint_LoadBalancerOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std::option::Option<std::vec::Vec<VerifiedaccessEndpoint_CidrOptions_PortRange>>,
    dynamic: VerifiedaccessEndpoint_LoadBalancerOptionsDynamic,
}

impl VerifiedaccessEndpoint_LoadBalancerOptions {
    #[doc = "Set the field `load_balancer_arn`.\n"]
    pub fn set_load_balancer_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.load_balancer_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnet_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VerifiedaccessEndpoint_CidrOptions_PortRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.port_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.port_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VerifiedaccessEndpoint_LoadBalancerOptions {
    type O = terrars::BlockAssignable<VerifiedaccessEndpoint_LoadBalancerOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessEndpoint_LoadBalancerOptions {}

impl BuildVerifiedaccessEndpoint_LoadBalancerOptions {
    pub fn build(self) -> VerifiedaccessEndpoint_LoadBalancerOptions {
        VerifiedaccessEndpoint_LoadBalancerOptions {
            load_balancer_arn: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
            port_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedaccessEndpoint_LoadBalancerOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedaccessEndpoint_LoadBalancerOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VerifiedaccessEndpoint_LoadBalancerOptionsRef {
        VerifiedaccessEndpoint_LoadBalancerOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessEndpoint_LoadBalancerOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `load_balancer_arn` after provisioning.\n"]
    pub fn load_balancer_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}

#[derive(Serialize, Default)]
struct VerifiedaccessEndpoint_NetworkInterfaceOptionsDynamic {
    port_range: std::option::Option<terrars::DynamicBlock<VerifiedaccessEndpoint_CidrOptions_PortRange>>,
}

#[derive(Serialize)]
pub struct VerifiedaccessEndpoint_NetworkInterfaceOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std::option::Option<std::vec::Vec<VerifiedaccessEndpoint_CidrOptions_PortRange>>,
    dynamic: VerifiedaccessEndpoint_NetworkInterfaceOptionsDynamic,
}

impl VerifiedaccessEndpoint_NetworkInterfaceOptions {
    #[doc = "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.network_interface_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VerifiedaccessEndpoint_CidrOptions_PortRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.port_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.port_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VerifiedaccessEndpoint_NetworkInterfaceOptions {
    type O = terrars::BlockAssignable<VerifiedaccessEndpoint_NetworkInterfaceOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessEndpoint_NetworkInterfaceOptions {}

impl BuildVerifiedaccessEndpoint_NetworkInterfaceOptions {
    pub fn build(self) -> VerifiedaccessEndpoint_NetworkInterfaceOptions {
        VerifiedaccessEndpoint_NetworkInterfaceOptions {
            network_interface_id: core::default::Default::default(),
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            port_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedaccessEndpoint_NetworkInterfaceOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedaccessEndpoint_NetworkInterfaceOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedaccessEndpoint_NetworkInterfaceOptionsRef {
        VerifiedaccessEndpoint_NetworkInterfaceOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessEndpoint_NetworkInterfaceOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedaccessEndpoint_RdsOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rds_db_cluster_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rds_db_instance_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rds_db_proxy_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rds_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl VerifiedaccessEndpoint_RdsOptions {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rds_db_cluster_arn`.\n"]
    pub fn set_rds_db_cluster_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.rds_db_cluster_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rds_db_instance_arn`.\n"]
    pub fn set_rds_db_instance_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.rds_db_instance_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rds_db_proxy_arn`.\n"]
    pub fn set_rds_db_proxy_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rds_db_proxy_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rds_endpoint`.\n"]
    pub fn set_rds_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rds_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnet_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VerifiedaccessEndpoint_RdsOptions {
    type O = terrars::BlockAssignable<VerifiedaccessEndpoint_RdsOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessEndpoint_RdsOptions {}

impl BuildVerifiedaccessEndpoint_RdsOptions {
    pub fn build(self) -> VerifiedaccessEndpoint_RdsOptions {
        VerifiedaccessEndpoint_RdsOptions {
            port: core::default::Default::default(),
            protocol: core::default::Default::default(),
            rds_db_cluster_arn: core::default::Default::default(),
            rds_db_instance_arn: core::default::Default::default(),
            rds_db_proxy_arn: core::default::Default::default(),
            rds_endpoint: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedaccessEndpoint_RdsOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedaccessEndpoint_RdsOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VerifiedaccessEndpoint_RdsOptionsRef {
        VerifiedaccessEndpoint_RdsOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessEndpoint_RdsOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `rds_db_cluster_arn` after provisioning.\n"]
    pub fn rds_db_cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rds_db_cluster_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `rds_db_instance_arn` after provisioning.\n"]
    pub fn rds_db_instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rds_db_instance_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `rds_db_proxy_arn` after provisioning.\n"]
    pub fn rds_db_proxy_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rds_db_proxy_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `rds_endpoint` after provisioning.\n"]
    pub fn rds_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rds_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedaccessEndpoint_SseSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_managed_key_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VerifiedaccessEndpoint_SseSpecification {
    #[doc = "Set the field `customer_managed_key_enabled`.\n"]
    pub fn set_customer_managed_key_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.customer_managed_key_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VerifiedaccessEndpoint_SseSpecification {
    type O = terrars::BlockAssignable<VerifiedaccessEndpoint_SseSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessEndpoint_SseSpecification {}

impl BuildVerifiedaccessEndpoint_SseSpecification {
    pub fn build(self) -> VerifiedaccessEndpoint_SseSpecification {
        VerifiedaccessEndpoint_SseSpecification {
            customer_managed_key_enabled: core::default::Default::default(),
            kms_key_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedaccessEndpoint_SseSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedaccessEndpoint_SseSpecificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VerifiedaccessEndpoint_SseSpecificationRef {
        VerifiedaccessEndpoint_SseSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessEndpoint_SseSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `customer_managed_key_enabled` after provisioning.\n"]
    pub fn customer_managed_key_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_managed_key_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedaccessEndpoint_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VerifiedaccessEndpoint_Timeouts {
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

impl terrars::ToListMappable for VerifiedaccessEndpoint_Timeouts {
    type O = terrars::BlockAssignable<VerifiedaccessEndpoint_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedaccessEndpoint_Timeouts {}

impl BuildVerifiedaccessEndpoint_Timeouts {
    pub fn build(self) -> VerifiedaccessEndpoint_Timeouts {
        VerifiedaccessEndpoint_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedaccessEndpoint_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedaccessEndpoint_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VerifiedaccessEndpoint_TimeoutsRef {
        VerifiedaccessEndpoint_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedaccessEndpoint_TimeoutsRef {
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
