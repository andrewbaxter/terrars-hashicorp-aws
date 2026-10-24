use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataElbData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataElb_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataElbData>,
}

#[derive(Clone)]
pub struct DataElb(std::rc::Rc<DataElb_>);

impl DataElb {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `access_logs` after provisioning.\n"]
    pub fn access_logs(&self) -> terrars::ListRef<DataElbAccessLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_draining` after provisioning.\n"]
    pub fn connection_draining(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_draining", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_draining_timeout` after provisioning.\n"]
    pub fn connection_draining_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_draining_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cross_zone_load_balancing` after provisioning.\n"]
    pub fn cross_zone_load_balancing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_zone_load_balancing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desync_mitigation_mode` after provisioning.\n"]
    pub fn desync_mitigation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desync_mitigation_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::ListRef<DataElbHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_timeout` after provisioning.\n"]
    pub fn idle_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `internal` after provisioning.\n"]
    pub fn internal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener` after provisioning.\n"]
    pub fn listener(&self) -> terrars::SetRef<DataElbListenerElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.listener", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_security_group` after provisioning.\n"]
    pub fn source_security_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_security_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_security_group_id` after provisioning.\n"]
    pub fn source_security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataElb {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataElb { }

impl terrars::ToListMappable for DataElb {
    type O = terrars::ListRef<DataElbRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataElb_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_elb".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataElb {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataElb {
    pub fn build(self, stack: &mut terrars::Stack) -> DataElb {
        let out = DataElb(std::rc::Rc::new(DataElb_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataElbData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataElbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataElbRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataElbRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `access_logs` after provisioning.\n"]
    pub fn access_logs(&self) -> terrars::ListRef<DataElbAccessLogsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.access_logs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_draining` after provisioning.\n"]
    pub fn connection_draining(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_draining", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_draining_timeout` after provisioning.\n"]
    pub fn connection_draining_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_draining_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cross_zone_load_balancing` after provisioning.\n"]
    pub fn cross_zone_load_balancing(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_zone_load_balancing", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desync_mitigation_mode` after provisioning.\n"]
    pub fn desync_mitigation_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desync_mitigation_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check` after provisioning.\n"]
    pub fn health_check(&self) -> terrars::ListRef<DataElbHealthCheckElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.health_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_timeout` after provisioning.\n"]
    pub fn idle_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `internal` after provisioning.\n"]
    pub fn internal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.internal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener` after provisioning.\n"]
    pub fn listener(&self) -> terrars::SetRef<DataElbListenerElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.listener", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_security_group` after provisioning.\n"]
    pub fn source_security_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_security_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_security_group_id` after provisioning.\n"]
    pub fn source_security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_security_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zone_id` after provisioning.\n"]
    pub fn zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataElbAccessLogsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
}

impl DataElbAccessLogsEl {
    #[doc = "Set the field `bucket`.\n"]
    pub fn set_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_prefix`.\n"]
    pub fn set_bucket_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataElbAccessLogsEl {
    type O = terrars::BlockAssignable<DataElbAccessLogsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataElbAccessLogsEl {}

impl BuildDataElbAccessLogsEl {
    pub fn build(self) -> DataElbAccessLogsEl {
        DataElbAccessLogsEl {
            bucket: core::default::Default::default(),
            bucket_prefix: core::default::Default::default(),
            enabled: core::default::Default::default(),
            interval: core::default::Default::default(),
        }
    }
}

pub struct DataElbAccessLogsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataElbAccessLogsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataElbAccessLogsElRef {
        DataElbAccessLogsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataElbAccessLogsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_prefix` after provisioning.\n"]
    pub fn bucket_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }
}

#[derive(Serialize)]
pub struct DataElbHealthCheckEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    healthy_threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unhealthy_threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl DataElbHealthCheckEl {
    #[doc = "Set the field `healthy_threshold`.\n"]
    pub fn set_healthy_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.healthy_threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unhealthy_threshold`.\n"]
    pub fn set_unhealthy_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.unhealthy_threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataElbHealthCheckEl {
    type O = terrars::BlockAssignable<DataElbHealthCheckEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataElbHealthCheckEl {}

impl BuildDataElbHealthCheckEl {
    pub fn build(self) -> DataElbHealthCheckEl {
        DataElbHealthCheckEl {
            healthy_threshold: core::default::Default::default(),
            interval: core::default::Default::default(),
            target: core::default::Default::default(),
            timeout: core::default::Default::default(),
            unhealthy_threshold: core::default::Default::default(),
        }
    }
}

pub struct DataElbHealthCheckElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataElbHealthCheckElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataElbHealthCheckElRef {
        DataElbHealthCheckElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataElbHealthCheckElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `healthy_threshold` after provisioning.\n"]
    pub fn healthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.healthy_threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `unhealthy_threshold` after provisioning.\n"]
    pub fn unhealthy_threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unhealthy_threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct DataElbListenerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lb_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lb_protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssl_certificate_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataElbListenerEl {
    #[doc = "Set the field `instance_port`.\n"]
    pub fn set_instance_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_protocol`.\n"]
    pub fn set_instance_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lb_port`.\n"]
    pub fn set_lb_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lb_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lb_protocol`.\n"]
    pub fn set_lb_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lb_protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssl_certificate_id`.\n"]
    pub fn set_ssl_certificate_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ssl_certificate_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataElbListenerEl {
    type O = terrars::BlockAssignable<DataElbListenerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataElbListenerEl {}

impl BuildDataElbListenerEl {
    pub fn build(self) -> DataElbListenerEl {
        DataElbListenerEl {
            instance_port: core::default::Default::default(),
            instance_protocol: core::default::Default::default(),
            lb_port: core::default::Default::default(),
            lb_protocol: core::default::Default::default(),
            ssl_certificate_id: core::default::Default::default(),
        }
    }
}

pub struct DataElbListenerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataElbListenerElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataElbListenerElRef {
        DataElbListenerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataElbListenerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_port` after provisioning.\n"]
    pub fn instance_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_port", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_protocol` after provisioning.\n"]
    pub fn instance_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `lb_port` after provisioning.\n"]
    pub fn lb_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lb_port", self.base))
    }

    #[doc = "Get a reference to the value of field `lb_protocol` after provisioning.\n"]
    pub fn lb_protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lb_protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `ssl_certificate_id` after provisioning.\n"]
    pub fn ssl_certificate_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssl_certificate_id", self.base))
    }
}
