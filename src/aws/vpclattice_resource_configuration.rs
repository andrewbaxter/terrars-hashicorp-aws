#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VpclatticeResourceConfigurationData_ {
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
    allow_association_to_shareable_service_network: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_verification_id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_ranges: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_configuration_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_gateway_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_configuration_definition: std
    ::option
    ::Option<std::vec::Vec<VpclatticeResourceConfiguration_ResourceConfigurationDefinition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VpclatticeResourceConfiguration_Timeouts>,
    dynamic: VpclatticeResourceConfigurationDynamic,
}

struct VpclatticeResourceConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VpclatticeResourceConfigurationData_>,
}

#[derive(Clone)]
pub struct VpclatticeResourceConfiguration(std::rc::Rc<VpclatticeResourceConfiguration_>);

impl VpclatticeResourceConfiguration {
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

    #[doc = "Set the field `allow_association_to_shareable_service_network`.\n"]
    pub fn set_allow_association_to_shareable_service_network(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().allow_association_to_shareable_service_network =
            std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_domain_name`.\n"]
    pub fn set_custom_domain_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().custom_domain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain_verification_id`.\n"]
    pub fn set_domain_verification_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().domain_verification_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_ranges`.\n"]
    pub fn set_port_ranges(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().port_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_configuration_group_id`.\n"]
    pub fn set_resource_configuration_group_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().resource_configuration_group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_gateway_identifier`.\n"]
    pub fn set_resource_gateway_identifier(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().resource_gateway_identifier = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_configuration_definition`.\n"]
    pub fn set_resource_configuration_definition(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VpclatticeResourceConfiguration_ResourceConfigurationDefinition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().resource_configuration_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.resource_configuration_definition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VpclatticeResourceConfiguration_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `allow_association_to_shareable_service_network` after provisioning.\n"]
    pub fn allow_association_to_shareable_service_network(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_association_to_shareable_service_network", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_domain_name` after provisioning.\n"]
    pub fn custom_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_verification_arn` after provisioning.\n"]
    pub fn domain_verification_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_verification_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_verification_id` after provisioning.\n"]
    pub fn domain_verification_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_verification_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_verification_status` after provisioning.\n"]
    pub fn domain_verification_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_verification_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port_ranges` after provisioning.\n"]
    pub fn port_ranges(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.port_ranges", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_configuration_group_id` after provisioning.\n"]
    pub fn resource_configuration_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_configuration_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_gateway_identifier` after provisioning.\n"]
    pub fn resource_gateway_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_gateway_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_configuration_definition` after provisioning.\n"]
    pub fn resource_configuration_definition(
        &self,
    ) -> terrars::ListRef<VpclatticeResourceConfiguration_ResourceConfigurationDefinitionRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.resource_configuration_definition", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpclatticeResourceConfiguration_TimeoutsRef {
        VpclatticeResourceConfiguration_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for VpclatticeResourceConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VpclatticeResourceConfiguration { }

impl terrars::ToListMappable for VpclatticeResourceConfiguration {
    type O = terrars::ListRef<VpclatticeResourceConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VpclatticeResourceConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_vpclattice_resource_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpclatticeResourceConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildVpclatticeResourceConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> VpclatticeResourceConfiguration {
        let out = VpclatticeResourceConfiguration(std::rc::Rc::new(VpclatticeResourceConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VpclatticeResourceConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                allow_association_to_shareable_service_network: core::default::Default::default(),
                custom_domain_name: core::default::Default::default(),
                domain_verification_id: core::default::Default::default(),
                name: self.name,
                port_ranges: core::default::Default::default(),
                protocol: core::default::Default::default(),
                region: core::default::Default::default(),
                resource_configuration_group_id: core::default::Default::default(),
                resource_gateway_identifier: core::default::Default::default(),
                tags: core::default::Default::default(),
                type_: core::default::Default::default(),
                resource_configuration_definition: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpclatticeResourceConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpclatticeResourceConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VpclatticeResourceConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allow_association_to_shareable_service_network` after provisioning.\n"]
    pub fn allow_association_to_shareable_service_network(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allow_association_to_shareable_service_network", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_domain_name` after provisioning.\n"]
    pub fn custom_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_verification_arn` after provisioning.\n"]
    pub fn domain_verification_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_verification_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_verification_id` after provisioning.\n"]
    pub fn domain_verification_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_verification_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_verification_status` after provisioning.\n"]
    pub fn domain_verification_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_verification_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `port_ranges` after provisioning.\n"]
    pub fn port_ranges(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.port_ranges", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_configuration_group_id` after provisioning.\n"]
    pub fn resource_configuration_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.resource_configuration_group_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `resource_gateway_identifier` after provisioning.\n"]
    pub fn resource_gateway_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_gateway_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_configuration_definition` after provisioning.\n"]
    pub fn resource_configuration_definition(
        &self,
    ) -> terrars::ListRef<VpclatticeResourceConfiguration_ResourceConfigurationDefinitionRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.resource_configuration_definition", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpclatticeResourceConfiguration_TimeoutsRef {
        VpclatticeResourceConfiguration_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct VpclatticeResourceConfigurationDynamic {
    resource_configuration_definition: std
    ::option
    ::Option<terrars::DynamicBlock<VpclatticeResourceConfiguration_ResourceConfigurationDefinition>>,
}

#[derive(Serialize)]
pub struct VpclatticeResourceConfiguration_ResourceConfigurationDefinition_ArnResource {
    arn: terrars::PrimField<std::string::String>,
}

impl VpclatticeResourceConfiguration_ResourceConfigurationDefinition_ArnResource { }

impl terrars::ToListMappable for VpclatticeResourceConfiguration_ResourceConfigurationDefinition_ArnResource {
    type O = terrars::BlockAssignable<VpclatticeResourceConfiguration_ResourceConfigurationDefinition_ArnResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeResourceConfiguration_ResourceConfigurationDefinition_ArnResource {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildVpclatticeResourceConfiguration_ResourceConfigurationDefinition_ArnResource {
    pub fn build(self) -> VpclatticeResourceConfiguration_ResourceConfigurationDefinition_ArnResource {
        VpclatticeResourceConfiguration_ResourceConfigurationDefinition_ArnResource { arn: self.arn }
    }
}

#[allow(dead_code)]
pub struct VpclatticeResourceConfiguration_ResourceConfigurationDefinition_ArnResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpclatticeResourceConfiguration_ResourceConfigurationDefinition_ArnResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VpclatticeResourceConfiguration_ResourceConfigurationDefinition_ArnResourceRef {
        VpclatticeResourceConfiguration_ResourceConfigurationDefinition_ArnResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeResourceConfiguration_ResourceConfigurationDefinition_ArnResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }
}

#[derive(Serialize)]
pub struct VpclatticeResourceConfiguration_ResourceConfigurationDefinition_DnsResource {
    domain_name: terrars::PrimField<std::string::String>,
    ip_address_type: terrars::PrimField<std::string::String>,
}

impl VpclatticeResourceConfiguration_ResourceConfigurationDefinition_DnsResource { }

impl terrars::ToListMappable for VpclatticeResourceConfiguration_ResourceConfigurationDefinition_DnsResource {
    type O = terrars::BlockAssignable<VpclatticeResourceConfiguration_ResourceConfigurationDefinition_DnsResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeResourceConfiguration_ResourceConfigurationDefinition_DnsResource {
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub ip_address_type: terrars::PrimField<std::string::String>,
}

impl BuildVpclatticeResourceConfiguration_ResourceConfigurationDefinition_DnsResource {
    pub fn build(self) -> VpclatticeResourceConfiguration_ResourceConfigurationDefinition_DnsResource {
        VpclatticeResourceConfiguration_ResourceConfigurationDefinition_DnsResource {
            domain_name: self.domain_name,
            ip_address_type: self.ip_address_type,
        }
    }
}

#[allow(dead_code)]
pub struct VpclatticeResourceConfiguration_ResourceConfigurationDefinition_DnsResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpclatticeResourceConfiguration_ResourceConfigurationDefinition_DnsResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VpclatticeResourceConfiguration_ResourceConfigurationDefinition_DnsResourceRef {
        VpclatticeResourceConfiguration_ResourceConfigurationDefinition_DnsResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeResourceConfiguration_ResourceConfigurationDefinition_DnsResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.base))
    }
}

#[derive(Serialize)]
pub struct VpclatticeResourceConfiguration_ResourceConfigurationDefinition_IpResource {
    ip_address: terrars::PrimField<std::string::String>,
}

impl VpclatticeResourceConfiguration_ResourceConfigurationDefinition_IpResource { }

impl terrars::ToListMappable for VpclatticeResourceConfiguration_ResourceConfigurationDefinition_IpResource {
    type O = terrars::BlockAssignable<VpclatticeResourceConfiguration_ResourceConfigurationDefinition_IpResource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeResourceConfiguration_ResourceConfigurationDefinition_IpResource {
    #[doc = ""]
    pub ip_address: terrars::PrimField<std::string::String>,
}

impl BuildVpclatticeResourceConfiguration_ResourceConfigurationDefinition_IpResource {
    pub fn build(self) -> VpclatticeResourceConfiguration_ResourceConfigurationDefinition_IpResource {
        VpclatticeResourceConfiguration_ResourceConfigurationDefinition_IpResource { ip_address: self.ip_address }
    }
}

#[allow(dead_code)]
pub struct VpclatticeResourceConfiguration_ResourceConfigurationDefinition_IpResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpclatticeResourceConfiguration_ResourceConfigurationDefinition_IpResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VpclatticeResourceConfiguration_ResourceConfigurationDefinition_IpResourceRef {
        VpclatticeResourceConfiguration_ResourceConfigurationDefinition_IpResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeResourceConfiguration_ResourceConfigurationDefinition_IpResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_address` after provisioning.\n"]
    pub fn ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpclatticeResourceConfiguration_ResourceConfigurationDefinitionDynamic {
    arn_resource: std
    ::option
    ::Option<terrars::DynamicBlock<VpclatticeResourceConfiguration_ResourceConfigurationDefinition_ArnResource>>,
    dns_resource: std
    ::option
    ::Option<terrars::DynamicBlock<VpclatticeResourceConfiguration_ResourceConfigurationDefinition_DnsResource>>,
    ip_resource: std
    ::option
    ::Option<terrars::DynamicBlock<VpclatticeResourceConfiguration_ResourceConfigurationDefinition_IpResource>>,
}

#[derive(Serialize)]
pub struct VpclatticeResourceConfiguration_ResourceConfigurationDefinition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn_resource: std
    ::option
    ::Option<std::vec::Vec<VpclatticeResourceConfiguration_ResourceConfigurationDefinition_ArnResource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_resource: std
    ::option
    ::Option<std::vec::Vec<VpclatticeResourceConfiguration_ResourceConfigurationDefinition_DnsResource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_resource: std
    ::option
    ::Option<std::vec::Vec<VpclatticeResourceConfiguration_ResourceConfigurationDefinition_IpResource>>,
    dynamic: VpclatticeResourceConfiguration_ResourceConfigurationDefinitionDynamic,
}

impl VpclatticeResourceConfiguration_ResourceConfigurationDefinition {
    #[doc = "Set the field `arn_resource`.\n"]
    pub fn set_arn_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VpclatticeResourceConfiguration_ResourceConfigurationDefinition_ArnResource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.arn_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.arn_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dns_resource`.\n"]
    pub fn set_dns_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VpclatticeResourceConfiguration_ResourceConfigurationDefinition_DnsResource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dns_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dns_resource = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ip_resource`.\n"]
    pub fn set_ip_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<VpclatticeResourceConfiguration_ResourceConfigurationDefinition_IpResource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VpclatticeResourceConfiguration_ResourceConfigurationDefinition {
    type O = terrars::BlockAssignable<VpclatticeResourceConfiguration_ResourceConfigurationDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeResourceConfiguration_ResourceConfigurationDefinition {}

impl BuildVpclatticeResourceConfiguration_ResourceConfigurationDefinition {
    pub fn build(self) -> VpclatticeResourceConfiguration_ResourceConfigurationDefinition {
        VpclatticeResourceConfiguration_ResourceConfigurationDefinition {
            arn_resource: core::default::Default::default(),
            dns_resource: core::default::Default::default(),
            ip_resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpclatticeResourceConfiguration_ResourceConfigurationDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpclatticeResourceConfiguration_ResourceConfigurationDefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VpclatticeResourceConfiguration_ResourceConfigurationDefinitionRef {
        VpclatticeResourceConfiguration_ResourceConfigurationDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeResourceConfiguration_ResourceConfigurationDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn_resource` after provisioning.\n"]
    pub fn arn_resource(
        &self,
    ) -> terrars::ListRef<VpclatticeResourceConfiguration_ResourceConfigurationDefinition_ArnResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.arn_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_resource` after provisioning.\n"]
    pub fn dns_resource(
        &self,
    ) -> terrars::ListRef<VpclatticeResourceConfiguration_ResourceConfigurationDefinition_DnsResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_resource", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_resource` after provisioning.\n"]
    pub fn ip_resource(
        &self,
    ) -> terrars::ListRef<VpclatticeResourceConfiguration_ResourceConfigurationDefinition_IpResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_resource", self.base))
    }
}

#[derive(Serialize)]
pub struct VpclatticeResourceConfiguration_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpclatticeResourceConfiguration_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpclatticeResourceConfiguration_Timeouts {
    type O = terrars::BlockAssignable<VpclatticeResourceConfiguration_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeResourceConfiguration_Timeouts {}

impl BuildVpclatticeResourceConfiguration_Timeouts {
    pub fn build(self) -> VpclatticeResourceConfiguration_Timeouts {
        VpclatticeResourceConfiguration_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpclatticeResourceConfiguration_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpclatticeResourceConfiguration_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpclatticeResourceConfiguration_TimeoutsRef {
        VpclatticeResourceConfiguration_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeResourceConfiguration_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
