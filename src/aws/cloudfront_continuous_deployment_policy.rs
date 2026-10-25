#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudfrontContinuousDeploymentPolicyData_ {
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
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    staging_distribution_dns_names: std
    ::option
    ::Option<std::vec::Vec<CloudfrontContinuousDeploymentPolicy_StagingDistributionDnsNames>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_config: std::option::Option<std::vec::Vec<CloudfrontContinuousDeploymentPolicy_TrafficConfig>>,
    dynamic: CloudfrontContinuousDeploymentPolicyDynamic,
}

struct CloudfrontContinuousDeploymentPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudfrontContinuousDeploymentPolicyData_>,
}

#[derive(Clone)]
pub struct CloudfrontContinuousDeploymentPolicy(std::rc::Rc<CloudfrontContinuousDeploymentPolicy_>);

impl CloudfrontContinuousDeploymentPolicy {
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

    #[doc = "Set the field `staging_distribution_dns_names`.\n"]
    pub fn set_staging_distribution_dns_names(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CloudfrontContinuousDeploymentPolicy_StagingDistributionDnsNames>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().staging_distribution_dns_names = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.staging_distribution_dns_names = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `traffic_config`.\n"]
    pub fn set_traffic_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudfrontContinuousDeploymentPolicy_TrafficConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().traffic_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.traffic_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\n"]
    pub fn last_modified_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `staging_distribution_dns_names` after provisioning.\n"]
    pub fn staging_distribution_dns_names(
        &self,
    ) -> terrars::ListRef<CloudfrontContinuousDeploymentPolicy_StagingDistributionDnsNamesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.staging_distribution_dns_names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `traffic_config` after provisioning.\n"]
    pub fn traffic_config(&self) -> terrars::ListRef<CloudfrontContinuousDeploymentPolicy_TrafficConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic_config", self.extract_ref()))
    }
}

impl terrars::Referable for CloudfrontContinuousDeploymentPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudfrontContinuousDeploymentPolicy { }

impl terrars::ToListMappable for CloudfrontContinuousDeploymentPolicy {
    type O = terrars::ListRef<CloudfrontContinuousDeploymentPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudfrontContinuousDeploymentPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudfront_continuous_deployment_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudfrontContinuousDeploymentPolicy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildCloudfrontContinuousDeploymentPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudfrontContinuousDeploymentPolicy {
        let out = CloudfrontContinuousDeploymentPolicy(std::rc::Rc::new(CloudfrontContinuousDeploymentPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudfrontContinuousDeploymentPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                enabled: self.enabled,
                staging_distribution_dns_names: core::default::Default::default(),
                traffic_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudfrontContinuousDeploymentPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontContinuousDeploymentPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudfrontContinuousDeploymentPolicyRef {
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

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `etag` after provisioning.\n"]
    pub fn etag(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.etag", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\n"]
    pub fn last_modified_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `staging_distribution_dns_names` after provisioning.\n"]
    pub fn staging_distribution_dns_names(
        &self,
    ) -> terrars::ListRef<CloudfrontContinuousDeploymentPolicy_StagingDistributionDnsNamesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.staging_distribution_dns_names", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `traffic_config` after provisioning.\n"]
    pub fn traffic_config(&self) -> terrars::ListRef<CloudfrontContinuousDeploymentPolicy_TrafficConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontContinuousDeploymentPolicyDynamic {
    staging_distribution_dns_names: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontContinuousDeploymentPolicy_StagingDistributionDnsNames>>,
    traffic_config: std::option::Option<terrars::DynamicBlock<CloudfrontContinuousDeploymentPolicy_TrafficConfig>>,
}

#[derive(Serialize)]
pub struct CloudfrontContinuousDeploymentPolicy_StagingDistributionDnsNames {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    items: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    quantity: terrars::PrimField<f64>,
}

impl CloudfrontContinuousDeploymentPolicy_StagingDistributionDnsNames {
    #[doc = "Set the field `items`.\n"]
    pub fn set_items(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.items = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CloudfrontContinuousDeploymentPolicy_StagingDistributionDnsNames {
    type O = terrars::BlockAssignable<CloudfrontContinuousDeploymentPolicy_StagingDistributionDnsNames>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontContinuousDeploymentPolicy_StagingDistributionDnsNames {
    #[doc = ""]
    pub quantity: terrars::PrimField<f64>,
}

impl BuildCloudfrontContinuousDeploymentPolicy_StagingDistributionDnsNames {
    pub fn build(self) -> CloudfrontContinuousDeploymentPolicy_StagingDistributionDnsNames {
        CloudfrontContinuousDeploymentPolicy_StagingDistributionDnsNames {
            items: core::default::Default::default(),
            quantity: self.quantity,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontContinuousDeploymentPolicy_StagingDistributionDnsNamesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontContinuousDeploymentPolicy_StagingDistributionDnsNamesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontContinuousDeploymentPolicy_StagingDistributionDnsNamesRef {
        CloudfrontContinuousDeploymentPolicy_StagingDistributionDnsNamesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontContinuousDeploymentPolicy_StagingDistributionDnsNamesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `items` after provisioning.\n"]
    pub fn items(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.items", self.base))
    }

    #[doc = "Get a reference to the value of field `quantity` after provisioning.\n"]
    pub fn quantity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quantity", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleHeaderConfig {
    header: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleHeaderConfig { }

impl terrars::ToListMappable for CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleHeaderConfig {
    type O = terrars::BlockAssignable<CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleHeaderConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleHeaderConfig {
    #[doc = ""]
    pub header: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleHeaderConfig {
    pub fn build(self) -> CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleHeaderConfig {
        CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleHeaderConfig {
            header: self.header,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleHeaderConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleHeaderConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleHeaderConfigRef {
        CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleHeaderConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleHeaderConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig_SessionStickinessConfig {
    idle_ttl: terrars::PrimField<f64>,
    maximum_ttl: terrars::PrimField<f64>,
}

impl CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig_SessionStickinessConfig { }

impl terrars
::ToListMappable for CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig_SessionStickinessConfig {
    type O =
        terrars
        ::BlockAssignable<
            CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig_SessionStickinessConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig_SessionStickinessConfig {
    #[doc = ""]
    pub idle_ttl: terrars::PrimField<f64>,
    #[doc = ""]
    pub maximum_ttl: terrars::PrimField<f64>,
}

impl BuildCloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig_SessionStickinessConfig {
    pub fn build(
        self,
    ) -> CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig_SessionStickinessConfig {
        CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig_SessionStickinessConfig {
            idle_ttl: self.idle_ttl,
            maximum_ttl: self.maximum_ttl,
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig_SessionStickinessConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig_SessionStickinessConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig_SessionStickinessConfigRef {
        CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig_SessionStickinessConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig_SessionStickinessConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_ttl` after provisioning.\n"]
    pub fn idle_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_ttl", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_ttl` after provisioning.\n"]
    pub fn maximum_ttl(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_ttl", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfigDynamic {
    session_stickiness_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig_SessionStickinessConfig>,
    >,
}

#[derive(Serialize)]
pub struct CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig {
    weight: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_stickiness_config: std
    ::option
    ::Option<
        std::vec::Vec<CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig_SessionStickinessConfig>,
    >,
    dynamic: CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfigDynamic,
}

impl CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig {
    #[doc = "Set the field `session_stickiness_config`.\n"]
    pub fn set_session_stickiness_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig_SessionStickinessConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.session_stickiness_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.session_stickiness_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig {
    type O = terrars::BlockAssignable<CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig {
    #[doc = ""]
    pub weight: terrars::PrimField<f64>,
}

impl BuildCloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig {
    pub fn build(self) -> CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig {
        CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig {
            weight: self.weight,
            session_stickiness_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfigRef {
        CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }

    #[doc = "Get a reference to the value of field `session_stickiness_config` after provisioning.\n"]
    pub fn session_stickiness_config(
        &self,
    ) -> terrars
    ::ListRef<CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig_SessionStickinessConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.session_stickiness_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudfrontContinuousDeploymentPolicy_TrafficConfigDynamic {
    single_header_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleHeaderConfig>>,
    single_weight_config: std
    ::option
    ::Option<terrars::DynamicBlock<CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig>>,
}

#[derive(Serialize)]
pub struct CloudfrontContinuousDeploymentPolicy_TrafficConfig {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_header_config: std
    ::option
    ::Option<std::vec::Vec<CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleHeaderConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_weight_config: std
    ::option
    ::Option<std::vec::Vec<CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig>>,
    dynamic: CloudfrontContinuousDeploymentPolicy_TrafficConfigDynamic,
}

impl CloudfrontContinuousDeploymentPolicy_TrafficConfig {
    #[doc = "Set the field `single_header_config`.\n"]
    pub fn set_single_header_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleHeaderConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.single_header_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.single_header_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `single_weight_config`.\n"]
    pub fn set_single_weight_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.single_weight_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.single_weight_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudfrontContinuousDeploymentPolicy_TrafficConfig {
    type O = terrars::BlockAssignable<CloudfrontContinuousDeploymentPolicy_TrafficConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudfrontContinuousDeploymentPolicy_TrafficConfig {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCloudfrontContinuousDeploymentPolicy_TrafficConfig {
    pub fn build(self) -> CloudfrontContinuousDeploymentPolicy_TrafficConfig {
        CloudfrontContinuousDeploymentPolicy_TrafficConfig {
            type_: self.type_,
            single_header_config: core::default::Default::default(),
            single_weight_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudfrontContinuousDeploymentPolicy_TrafficConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudfrontContinuousDeploymentPolicy_TrafficConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudfrontContinuousDeploymentPolicy_TrafficConfigRef {
        CloudfrontContinuousDeploymentPolicy_TrafficConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudfrontContinuousDeploymentPolicy_TrafficConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `single_header_config` after provisioning.\n"]
    pub fn single_header_config(
        &self,
    ) -> terrars::ListRef<CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleHeaderConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.single_header_config", self.base))
    }

    #[doc = "Get a reference to the value of field `single_weight_config` after provisioning.\n"]
    pub fn single_weight_config(
        &self,
    ) -> terrars::ListRef<CloudfrontContinuousDeploymentPolicy_TrafficConfig_SingleWeightConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.single_weight_config", self.base))
    }
}
