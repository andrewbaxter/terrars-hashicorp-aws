use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataGlobalacceleratorAcceleratorData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataGlobalacceleratorAccelerator_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataGlobalacceleratorAcceleratorData>,
}

#[derive(Clone)]
pub struct DataGlobalacceleratorAccelerator(std::rc::Rc<DataGlobalacceleratorAccelerator_>);

impl DataGlobalacceleratorAccelerator {
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

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> terrars::ListRef<DataGlobalacceleratorAcceleratorAttributesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_sets` after provisioning.\n"]
    pub fn ip_sets(&self) -> terrars::ListRef<DataGlobalacceleratorAcceleratorIpSetsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_sets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataGlobalacceleratorAccelerator {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataGlobalacceleratorAccelerator { }

impl terrars::ToListMappable for DataGlobalacceleratorAccelerator {
    type O = terrars::ListRef<DataGlobalacceleratorAcceleratorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataGlobalacceleratorAccelerator_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_globalaccelerator_accelerator".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataGlobalacceleratorAccelerator {
    pub tf_id: std::string::String,
}

impl BuildDataGlobalacceleratorAccelerator {
    pub fn build(self, stack: &mut terrars::Stack) -> DataGlobalacceleratorAccelerator {
        let out = DataGlobalacceleratorAccelerator(std::rc::Rc::new(DataGlobalacceleratorAccelerator_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataGlobalacceleratorAcceleratorData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataGlobalacceleratorAcceleratorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlobalacceleratorAcceleratorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataGlobalacceleratorAcceleratorRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attributes` after provisioning.\n"]
    pub fn attributes(&self) -> terrars::ListRef<DataGlobalacceleratorAcceleratorAttributesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hosted_zone_id` after provisioning.\n"]
    pub fn hosted_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hosted_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ip_sets` after provisioning.\n"]
    pub fn ip_sets(&self) -> terrars::ListRef<DataGlobalacceleratorAcceleratorIpSetsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_sets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataGlobalacceleratorAcceleratorAttributesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flow_logs_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flow_logs_s3_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flow_logs_s3_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGlobalacceleratorAcceleratorAttributesEl {
    #[doc = "Set the field `flow_logs_enabled`.\n"]
    pub fn set_flow_logs_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.flow_logs_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flow_logs_s3_bucket`.\n"]
    pub fn set_flow_logs_s3_bucket(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.flow_logs_s3_bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `flow_logs_s3_prefix`.\n"]
    pub fn set_flow_logs_s3_prefix(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.flow_logs_s3_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlobalacceleratorAcceleratorAttributesEl {
    type O = terrars::BlockAssignable<DataGlobalacceleratorAcceleratorAttributesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlobalacceleratorAcceleratorAttributesEl {}

impl BuildDataGlobalacceleratorAcceleratorAttributesEl {
    pub fn build(self) -> DataGlobalacceleratorAcceleratorAttributesEl {
        DataGlobalacceleratorAcceleratorAttributesEl {
            flow_logs_enabled: core::default::Default::default(),
            flow_logs_s3_bucket: core::default::Default::default(),
            flow_logs_s3_prefix: core::default::Default::default(),
        }
    }
}

pub struct DataGlobalacceleratorAcceleratorAttributesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlobalacceleratorAcceleratorAttributesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataGlobalacceleratorAcceleratorAttributesElRef {
        DataGlobalacceleratorAcceleratorAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlobalacceleratorAcceleratorAttributesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `flow_logs_enabled` after provisioning.\n"]
    pub fn flow_logs_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flow_logs_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `flow_logs_s3_bucket` after provisioning.\n"]
    pub fn flow_logs_s3_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flow_logs_s3_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `flow_logs_s3_prefix` after provisioning.\n"]
    pub fn flow_logs_s3_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flow_logs_s3_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGlobalacceleratorAcceleratorIpSetsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_family: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGlobalacceleratorAcceleratorIpSetsEl {
    #[doc = "Set the field `ip_addresses`.\n"]
    pub fn set_ip_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ip_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_family`.\n"]
    pub fn set_ip_family(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_family = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlobalacceleratorAcceleratorIpSetsEl {
    type O = terrars::BlockAssignable<DataGlobalacceleratorAcceleratorIpSetsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlobalacceleratorAcceleratorIpSetsEl {}

impl BuildDataGlobalacceleratorAcceleratorIpSetsEl {
    pub fn build(self) -> DataGlobalacceleratorAcceleratorIpSetsEl {
        DataGlobalacceleratorAcceleratorIpSetsEl {
            ip_addresses: core::default::Default::default(),
            ip_family: core::default::Default::default(),
        }
    }
}

pub struct DataGlobalacceleratorAcceleratorIpSetsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlobalacceleratorAcceleratorIpSetsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGlobalacceleratorAcceleratorIpSetsElRef {
        DataGlobalacceleratorAcceleratorIpSetsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlobalacceleratorAcceleratorIpSetsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_addresses` after provisioning.\n"]
    pub fn ip_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_family` after provisioning.\n"]
    pub fn ip_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_family", self.base))
    }
}
