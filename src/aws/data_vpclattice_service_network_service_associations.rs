#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataVpclatticeServiceNetworkServiceAssociationsData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_network_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataVpclatticeServiceNetworkServiceAssociations_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVpclatticeServiceNetworkServiceAssociationsData_>,
}

#[derive(Clone)]
pub struct DataVpclatticeServiceNetworkServiceAssociations(
    std::rc::Rc<DataVpclatticeServiceNetworkServiceAssociations_>,
);

impl DataVpclatticeServiceNetworkServiceAssociations {
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

    #[doc = "Set the field `service_identifier`.\nID or ARN of the Service."]
    pub fn set_service_identifier(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_network_identifier`.\nID or ARN of the Service Network."]
    pub fn set_service_network_identifier(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().service_network_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::ListRef<DataVpclatticeServiceNetworkServiceAssociations_ItemsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_identifier` after provisioning.\nID or ARN of the Service."]
    pub fn service_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_identifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_network_identifier` after provisioning.\nID or ARN of the Service Network."]
    pub fn service_network_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_network_identifier", self.extract_ref()))
    }
}

impl terrars::Referable for DataVpclatticeServiceNetworkServiceAssociations {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataVpclatticeServiceNetworkServiceAssociations { }

impl terrars::ToListMappable for DataVpclatticeServiceNetworkServiceAssociations {
    type O = terrars::ListRef<DataVpclatticeServiceNetworkServiceAssociationsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataVpclatticeServiceNetworkServiceAssociations_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_vpclattice_service_network_service_associations".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVpclatticeServiceNetworkServiceAssociations {
    pub tf_id: std::string::String,
}

impl BuildDataVpclatticeServiceNetworkServiceAssociations {
    pub fn build(self, stack: &mut terrars::Stack) -> DataVpclatticeServiceNetworkServiceAssociations {
        let out =
            DataVpclatticeServiceNetworkServiceAssociations(
                std::rc::Rc::new(DataVpclatticeServiceNetworkServiceAssociations_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(DataVpclatticeServiceNetworkServiceAssociationsData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        region: core::default::Default::default(),
                        service_identifier: core::default::Default::default(),
                        service_network_identifier: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVpclatticeServiceNetworkServiceAssociationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpclatticeServiceNetworkServiceAssociationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVpclatticeServiceNetworkServiceAssociationsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::ListRef<DataVpclatticeServiceNetworkServiceAssociations_ItemsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.items", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_identifier` after provisioning.\nID or ARN of the Service."]
    pub fn service_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_identifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `service_network_identifier` after provisioning.\nID or ARN of the Service Network."]
    pub fn service_network_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_network_identifier", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataVpclatticeServiceNetworkServiceAssociations_Items_DnsEntry {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hosted_zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpclatticeServiceNetworkServiceAssociations_Items_DnsEntry {
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hosted_zone_id`.\n"]
    pub fn set_hosted_zone_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hosted_zone_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpclatticeServiceNetworkServiceAssociations_Items_DnsEntry {
    type O = terrars::BlockAssignable<DataVpclatticeServiceNetworkServiceAssociations_Items_DnsEntry>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpclatticeServiceNetworkServiceAssociations_Items_DnsEntry {}

impl BuildDataVpclatticeServiceNetworkServiceAssociations_Items_DnsEntry {
    pub fn build(self) -> DataVpclatticeServiceNetworkServiceAssociations_Items_DnsEntry {
        DataVpclatticeServiceNetworkServiceAssociations_Items_DnsEntry {
            domain_name: core::default::Default::default(),
            hosted_zone_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVpclatticeServiceNetworkServiceAssociations_Items_DnsEntryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpclatticeServiceNetworkServiceAssociations_Items_DnsEntryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVpclatticeServiceNetworkServiceAssociations_Items_DnsEntryRef {
        DataVpclatticeServiceNetworkServiceAssociations_Items_DnsEntryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpclatticeServiceNetworkServiceAssociations_Items_DnsEntryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpclatticeServiceNetworkServiceAssociations_Items {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_entry: std::option::Option<terrars::ListField<DataVpclatticeServiceNetworkServiceAssociations_Items_DnsEntry>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_network_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_network_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_network_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpclatticeServiceNetworkServiceAssociations_Items {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_by`.\n"]
    pub fn set_created_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_by = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_domain_name`.\n"]
    pub fn set_custom_domain_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.custom_domain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dns_entry`.\n"]
    pub fn set_dns_entry(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataVpclatticeServiceNetworkServiceAssociations_Items_DnsEntry>>,
    ) -> Self {
        self.dns_entry = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_arn`.\n"]
    pub fn set_service_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_id`.\n"]
    pub fn set_service_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_name`.\n"]
    pub fn set_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `service_network_id`.\n"]
    pub fn set_service_network_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_network_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpclatticeServiceNetworkServiceAssociations_Items {
    type O = terrars::BlockAssignable<DataVpclatticeServiceNetworkServiceAssociations_Items>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpclatticeServiceNetworkServiceAssociations_Items {}

impl BuildDataVpclatticeServiceNetworkServiceAssociations_Items {
    pub fn build(self) -> DataVpclatticeServiceNetworkServiceAssociations_Items {
        DataVpclatticeServiceNetworkServiceAssociations_Items {
            arn: core::default::Default::default(),
            created_at: core::default::Default::default(),
            created_by: core::default::Default::default(),
            custom_domain_name: core::default::Default::default(),
            dns_entry: core::default::Default::default(),
            id: core::default::Default::default(),
            service_arn: core::default::Default::default(),
            service_id: core::default::Default::default(),
            service_name: core::default::Default::default(),
            service_network_arn: core::default::Default::default(),
            service_network_id: core::default::Default::default(),
            service_network_name: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataVpclatticeServiceNetworkServiceAssociations_ItemsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpclatticeServiceNetworkServiceAssociations_ItemsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataVpclatticeServiceNetworkServiceAssociations_ItemsRef {
        DataVpclatticeServiceNetworkServiceAssociations_ItemsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpclatticeServiceNetworkServiceAssociations_ItemsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_domain_name` after provisioning.\n"]
    pub fn custom_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_entry` after provisioning.\n"]
    pub fn dns_entry(&self) -> terrars::ListRef<DataVpclatticeServiceNetworkServiceAssociations_Items_DnsEntryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dns_entry", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `service_arn` after provisioning.\n"]
    pub fn service_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `service_id` after provisioning.\n"]
    pub fn service_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_id", self.base))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }

    #[doc = "Get a reference to the value of field `service_network_arn` after provisioning.\n"]
    pub fn service_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_network_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `service_network_id` after provisioning.\n"]
    pub fn service_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_network_id", self.base))
    }

    #[doc = "Get a reference to the value of field `service_network_name` after provisioning.\n"]
    pub fn service_network_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_network_name", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
