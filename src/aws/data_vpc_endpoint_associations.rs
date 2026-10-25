#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataVpcEndpointAssociationsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    vpc_endpoint_id: terrars::PrimField<std::string::String>,
}

struct DataVpcEndpointAssociations_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVpcEndpointAssociationsData_>,
}

#[derive(Clone)]
pub struct DataVpcEndpointAssociations(std::rc::Rc<DataVpcEndpointAssociations_>);

impl DataVpcEndpointAssociations {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `associations` after provisioning.\n"]
    pub fn associations(&self) -> terrars::ListRef<DataVpcEndpointAssociations_AssociationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.associations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataVpcEndpointAssociations {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataVpcEndpointAssociations { }

impl terrars::ToListMappable for DataVpcEndpointAssociations {
    type O = terrars::ListRef<DataVpcEndpointAssociationsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataVpcEndpointAssociations_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_vpc_endpoint_associations".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVpcEndpointAssociations {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub vpc_endpoint_id: terrars::PrimField<std::string::String>,
}

impl BuildDataVpcEndpointAssociations {
    pub fn build(self, stack: &mut terrars::Stack) -> DataVpcEndpointAssociations {
        let out = DataVpcEndpointAssociations(std::rc::Rc::new(DataVpcEndpointAssociations_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataVpcEndpointAssociationsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                region: core::default::Default::default(),
                vpc_endpoint_id: self.vpc_endpoint_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVpcEndpointAssociationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcEndpointAssociationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVpcEndpointAssociationsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `associations` after provisioning.\n"]
    pub fn associations(&self) -> terrars::ListRef<DataVpcEndpointAssociations_AssociationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.associations", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_id` after provisioning.\n"]
    pub fn vpc_endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataVpcEndpointAssociations_Associations_DnsEntry {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosted_zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcEndpointAssociations_Associations_DnsEntry {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hosted_zone_id`.\n"]
    pub fn set_hosted_zone_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hosted_zone_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcEndpointAssociations_Associations_DnsEntry {
    type O = terrars::BlockAssignable<DataVpcEndpointAssociations_Associations_DnsEntry>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcEndpointAssociations_Associations_DnsEntry {}

impl BuildDataVpcEndpointAssociations_Associations_DnsEntry {
    pub fn build(self) -> DataVpcEndpointAssociations_Associations_DnsEntry {
        DataVpcEndpointAssociations_Associations_DnsEntry {
            dns_name: core::default::Default::default(),
            hosted_zone_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVpcEndpointAssociations_Associations_DnsEntryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcEndpointAssociations_Associations_DnsEntryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVpcEndpointAssociations_Associations_DnsEntryRef {
        DataVpcEndpointAssociations_Associations_DnsEntryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcEndpointAssociations_Associations_DnsEntryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcEndpointAssociations_Associations_PrivateDnsEntry {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosted_zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcEndpointAssociations_Associations_PrivateDnsEntry {
    #[doc = "Set the field `dns_name`.\n"]
    pub fn set_dns_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.dns_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hosted_zone_id`.\n"]
    pub fn set_hosted_zone_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hosted_zone_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcEndpointAssociations_Associations_PrivateDnsEntry {
    type O = terrars::BlockAssignable<DataVpcEndpointAssociations_Associations_PrivateDnsEntry>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcEndpointAssociations_Associations_PrivateDnsEntry {}

impl BuildDataVpcEndpointAssociations_Associations_PrivateDnsEntry {
    pub fn build(self) -> DataVpcEndpointAssociations_Associations_PrivateDnsEntry {
        DataVpcEndpointAssociations_Associations_PrivateDnsEntry {
            dns_name: core::default::Default::default(),
            hosted_zone_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVpcEndpointAssociations_Associations_PrivateDnsEntryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcEndpointAssociations_Associations_PrivateDnsEntryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVpcEndpointAssociations_Associations_PrivateDnsEntryRef {
        DataVpcEndpointAssociations_Associations_PrivateDnsEntryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcEndpointAssociations_Associations_PrivateDnsEntryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcEndpointAssociations_Associations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    associated_resource_accessibility: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    associated_resource_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_entry: std::option::Option<terrars::ListField<DataVpcEndpointAssociations_Associations_DnsEntry>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_dns_entry: std::option::Option<terrars::ListField<DataVpcEndpointAssociations_Associations_PrivateDnsEntry>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_configuration_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_network_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_network_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataVpcEndpointAssociations_Associations {
    #[doc = "Set the field `associated_resource_accessibility`.\n"]
    pub fn set_associated_resource_accessibility(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.associated_resource_accessibility = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `associated_resource_arn`.\n"]
    pub fn set_associated_resource_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.associated_resource_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_entry`.\n"]
    pub fn set_dns_entry(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataVpcEndpointAssociations_Associations_DnsEntry>>,
    ) -> Self {
        self.dns_entry = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_dns_entry`.\n"]
    pub fn set_private_dns_entry(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataVpcEndpointAssociations_Associations_PrivateDnsEntry>>,
    ) -> Self {
        self.private_dns_entry = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_configuration_group_arn`.\n"]
    pub fn set_resource_configuration_group_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.resource_configuration_group_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_network_arn`.\n"]
    pub fn set_service_network_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_network_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_network_name`.\n"]
    pub fn set_service_network_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_network_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcEndpointAssociations_Associations {
    type O = terrars::BlockAssignable<DataVpcEndpointAssociations_Associations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcEndpointAssociations_Associations {}

impl BuildDataVpcEndpointAssociations_Associations {
    pub fn build(self) -> DataVpcEndpointAssociations_Associations {
        DataVpcEndpointAssociations_Associations {
            associated_resource_accessibility: core::default::Default::default(),
            associated_resource_arn: core::default::Default::default(),
            dns_entry: core::default::Default::default(),
            id: core::default::Default::default(),
            private_dns_entry: core::default::Default::default(),
            resource_configuration_group_arn: core::default::Default::default(),
            service_network_arn: core::default::Default::default(),
            service_network_name: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVpcEndpointAssociations_AssociationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcEndpointAssociations_AssociationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcEndpointAssociations_AssociationsRef {
        DataVpcEndpointAssociations_AssociationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcEndpointAssociations_AssociationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `associated_resource_accessibility` after provisioning.\n"]
    pub fn associated_resource_accessibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.associated_resource_accessibility", self.base))
    }

    #[doc = "Get a reference to the value of field `associated_resource_arn` after provisioning.\n"]
    pub fn associated_resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.associated_resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_entry` after provisioning.\n"]
    pub fn dns_entry(&self) -> terrars::ListRef<DataVpcEndpointAssociations_Associations_DnsEntryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_entry", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `private_dns_entry` after provisioning.\n"]
    pub fn private_dns_entry(&self) -> terrars::ListRef<DataVpcEndpointAssociations_Associations_PrivateDnsEntryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_dns_entry", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_configuration_group_arn` after provisioning.\n"]
    pub fn resource_configuration_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_configuration_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `service_network_arn` after provisioning.\n"]
    pub fn service_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_network_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `service_network_name` after provisioning.\n"]
    pub fn service_network_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_network_name", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
