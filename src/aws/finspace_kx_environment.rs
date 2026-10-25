#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FinspaceKxEnvironmentData_ {
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
    kms_key_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_dns_configuration: std::option::Option<std::vec::Vec<FinspaceKxEnvironment_CustomDnsConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FinspaceKxEnvironment_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_configuration: std::option::Option<std::vec::Vec<FinspaceKxEnvironment_TransitGatewayConfiguration>>,
    dynamic: FinspaceKxEnvironmentDynamic,
}

struct FinspaceKxEnvironment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FinspaceKxEnvironmentData_>,
}

#[derive(Clone)]
pub struct FinspaceKxEnvironment(std::rc::Rc<FinspaceKxEnvironment_>);

impl FinspaceKxEnvironment {
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

    #[doc = "Set the field `custom_dns_configuration`.\n"]
    pub fn set_custom_dns_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FinspaceKxEnvironment_CustomDnsConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_dns_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_dns_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FinspaceKxEnvironment_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_configuration`.\n"]
    pub fn set_transit_gateway_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FinspaceKxEnvironment_TransitGatewayConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().transit_gateway_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.transit_gateway_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_timestamp` after provisioning.\n"]
    pub fn created_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `infrastructure_account_id` after provisioning.\n"]
    pub fn infrastructure_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_timestamp` after provisioning.\n"]
    pub fn last_modified_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `custom_dns_configuration` after provisioning.\n"]
    pub fn custom_dns_configuration(&self) -> terrars::ListRef<FinspaceKxEnvironment_CustomDnsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_dns_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FinspaceKxEnvironment_TimeoutsRef {
        FinspaceKxEnvironment_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_configuration` after provisioning.\n"]
    pub fn transit_gateway_configuration(
        &self,
    ) -> terrars::ListRef<FinspaceKxEnvironment_TransitGatewayConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for FinspaceKxEnvironment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FinspaceKxEnvironment { }

impl terrars::ToListMappable for FinspaceKxEnvironment {
    type O = terrars::ListRef<FinspaceKxEnvironmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FinspaceKxEnvironment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_finspace_kx_environment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFinspaceKxEnvironment {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub kms_key_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildFinspaceKxEnvironment {
    pub fn build(self, stack: &mut terrars::Stack) -> FinspaceKxEnvironment {
        let out = FinspaceKxEnvironment(std::rc::Rc::new(FinspaceKxEnvironment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FinspaceKxEnvironmentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                kms_key_id: self.kms_key_id,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                custom_dns_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                transit_gateway_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FinspaceKxEnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FinspaceKxEnvironmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FinspaceKxEnvironmentRef {
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

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_timestamp` after provisioning.\n"]
    pub fn created_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `infrastructure_account_id` after provisioning.\n"]
    pub fn infrastructure_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_timestamp` after provisioning.\n"]
    pub fn last_modified_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `custom_dns_configuration` after provisioning.\n"]
    pub fn custom_dns_configuration(&self) -> terrars::ListRef<FinspaceKxEnvironment_CustomDnsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_dns_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FinspaceKxEnvironment_TimeoutsRef {
        FinspaceKxEnvironment_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_configuration` after provisioning.\n"]
    pub fn transit_gateway_configuration(
        &self,
    ) -> terrars::ListRef<FinspaceKxEnvironment_TransitGatewayConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct FinspaceKxEnvironmentDynamic {
    custom_dns_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FinspaceKxEnvironment_CustomDnsConfiguration>>,
    transit_gateway_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FinspaceKxEnvironment_TransitGatewayConfiguration>>,
}

#[derive(Serialize)]
pub struct FinspaceKxEnvironment_CustomDnsConfiguration {
    custom_dns_server_ip: terrars::PrimField<std::string::String>,
    custom_dns_server_name: terrars::PrimField<std::string::String>,
}

impl FinspaceKxEnvironment_CustomDnsConfiguration { }

impl terrars::ToListMappable for FinspaceKxEnvironment_CustomDnsConfiguration {
    type O = terrars::BlockAssignable<FinspaceKxEnvironment_CustomDnsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxEnvironment_CustomDnsConfiguration {
    #[doc = ""]
    pub custom_dns_server_ip: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub custom_dns_server_name: terrars::PrimField<std::string::String>,
}

impl BuildFinspaceKxEnvironment_CustomDnsConfiguration {
    pub fn build(self) -> FinspaceKxEnvironment_CustomDnsConfiguration {
        FinspaceKxEnvironment_CustomDnsConfiguration {
            custom_dns_server_ip: self.custom_dns_server_ip,
            custom_dns_server_name: self.custom_dns_server_name,
        }
    }
}

#[allow(dead_code)]
pub struct FinspaceKxEnvironment_CustomDnsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FinspaceKxEnvironment_CustomDnsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FinspaceKxEnvironment_CustomDnsConfigurationRef {
        FinspaceKxEnvironment_CustomDnsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxEnvironment_CustomDnsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_dns_server_ip` after provisioning.\n"]
    pub fn custom_dns_server_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_dns_server_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_dns_server_name` after provisioning.\n"]
    pub fn custom_dns_server_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_dns_server_name", self.base))
    }
}

#[derive(Serialize)]
pub struct FinspaceKxEnvironment_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FinspaceKxEnvironment_Timeouts {
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

impl terrars::ToListMappable for FinspaceKxEnvironment_Timeouts {
    type O = terrars::BlockAssignable<FinspaceKxEnvironment_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxEnvironment_Timeouts {}

impl BuildFinspaceKxEnvironment_Timeouts {
    pub fn build(self) -> FinspaceKxEnvironment_Timeouts {
        FinspaceKxEnvironment_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FinspaceKxEnvironment_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FinspaceKxEnvironment_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FinspaceKxEnvironment_TimeoutsRef {
        FinspaceKxEnvironment_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxEnvironment_TimeoutsRef {
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

#[derive(Serialize)]
pub struct FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_IcmpTypeCode {
    code: terrars::PrimField<f64>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<f64>,
}

impl FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_IcmpTypeCode { }

impl terrars
::ToListMappable for FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_IcmpTypeCode {
    type O =
        terrars
        ::BlockAssignable<
            FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_IcmpTypeCode,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_IcmpTypeCode {
    #[doc = ""]
    pub code: terrars::PrimField<f64>,
    #[doc = ""]
    pub type_: terrars::PrimField<f64>,
}

impl BuildFinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_IcmpTypeCode {
    pub fn build(
        self,
    ) -> FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_IcmpTypeCode {
        FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_IcmpTypeCode {
            code: self.code,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_IcmpTypeCodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_IcmpTypeCodeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_IcmpTypeCodeRef {
        FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_IcmpTypeCodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_IcmpTypeCodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code` after provisioning.\n"]
    pub fn code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.code", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_PortRange {
    from: terrars::PrimField<f64>,
    to: terrars::PrimField<f64>,
}

impl FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_PortRange { }

impl terrars
::ToListMappable for FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_PortRange {
    type O =
        terrars
        ::BlockAssignable<
            FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_PortRange,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_PortRange {
    #[doc = ""]
    pub from: terrars::PrimField<f64>,
    #[doc = ""]
    pub to: terrars::PrimField<f64>,
}

impl BuildFinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_PortRange {
    pub fn build(
        self,
    ) -> FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_PortRange {
        FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_PortRange {
            from: self.from,
            to: self.to,
        }
    }
}

#[allow(dead_code)]
pub struct FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_PortRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_PortRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_PortRangeRef {
        FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_PortRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_PortRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize, Default)]
struct FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfigurationDynamic {
    icmp_type_code: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_IcmpTypeCode,
        >,
    >,
    port_range: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_PortRange>,
    >,
}

#[derive(Serialize)]
pub struct FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration {
    cidr_block: terrars::PrimField<std::string::String>,
    protocol: terrars::PrimField<std::string::String>,
    rule_action: terrars::PrimField<std::string::String>,
    rule_number: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    icmp_type_code: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_IcmpTypeCode>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std
    ::option
    ::Option<
        std::vec::Vec<FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_PortRange>,
    >,
    dynamic: FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfigurationDynamic,
}

impl FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration {
    #[doc = "Set the field `icmp_type_code`.\n"]
    pub fn set_icmp_type_code(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_IcmpTypeCode,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.icmp_type_code = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.icmp_type_code = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_PortRange,
                        >,
                    >,
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

impl terrars::ToListMappable for FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration {
    type O =
        terrars
        ::BlockAssignable<FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration {
    #[doc = ""]
    pub cidr_block: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub protocol: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule_action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule_number: terrars::PrimField<f64>,
}

impl BuildFinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration {
    pub fn build(self) -> FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration {
        FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration {
            cidr_block: self.cidr_block,
            protocol: self.protocol,
            rule_action: self.rule_action,
            rule_number: self.rule_number,
            icmp_type_code: core::default::Default::default(),
            port_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfigurationRef {
        FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_action` after provisioning.\n"]
    pub fn rule_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_action", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_number` after provisioning.\n"]
    pub fn rule_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_number", self.base))
    }

    #[doc = "Get a reference to the value of field `icmp_type_code` after provisioning.\n"]
    pub fn icmp_type_code(
        &self,
    ) -> terrars
    ::ListRef<FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_IcmpTypeCodeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.icmp_type_code", self.base))
    }

    #[doc = "Get a reference to the value of field `port_range` after provisioning.\n"]
    pub fn port_range(
        &self,
    ) -> terrars
    ::ListRef<FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration_PortRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_range", self.base))
    }
}

#[derive(Serialize, Default)]
struct FinspaceKxEnvironment_TransitGatewayConfigurationDynamic {
    attachment_network_acl_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct FinspaceKxEnvironment_TransitGatewayConfiguration {
    routable_cidr_space: terrars::PrimField<std::string::String>,
    transit_gateway_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attachment_network_acl_configuration: std
    ::option
    ::Option<std::vec::Vec<FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration>>,
    dynamic: FinspaceKxEnvironment_TransitGatewayConfigurationDynamic,
}

impl FinspaceKxEnvironment_TransitGatewayConfiguration {
    #[doc = "Set the field `attachment_network_acl_configuration`.\n"]
    pub fn set_attachment_network_acl_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.attachment_network_acl_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.attachment_network_acl_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FinspaceKxEnvironment_TransitGatewayConfiguration {
    type O = terrars::BlockAssignable<FinspaceKxEnvironment_TransitGatewayConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFinspaceKxEnvironment_TransitGatewayConfiguration {
    #[doc = ""]
    pub routable_cidr_space: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub transit_gateway_id: terrars::PrimField<std::string::String>,
}

impl BuildFinspaceKxEnvironment_TransitGatewayConfiguration {
    pub fn build(self) -> FinspaceKxEnvironment_TransitGatewayConfiguration {
        FinspaceKxEnvironment_TransitGatewayConfiguration {
            routable_cidr_space: self.routable_cidr_space,
            transit_gateway_id: self.transit_gateway_id,
            attachment_network_acl_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FinspaceKxEnvironment_TransitGatewayConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FinspaceKxEnvironment_TransitGatewayConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FinspaceKxEnvironment_TransitGatewayConfigurationRef {
        FinspaceKxEnvironment_TransitGatewayConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FinspaceKxEnvironment_TransitGatewayConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `routable_cidr_space` after provisioning.\n"]
    pub fn routable_cidr_space(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routable_cidr_space", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `attachment_network_acl_configuration` after provisioning.\n"]
    pub fn attachment_network_acl_configuration(
        &self,
    ) -> terrars::ListRef<FinspaceKxEnvironment_TransitGatewayConfiguration_AttachmentNetworkAclConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attachment_network_acl_configuration", self.base))
    }
}
