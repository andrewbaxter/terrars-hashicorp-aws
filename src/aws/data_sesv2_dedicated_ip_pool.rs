use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSesv2DedicatedIpPoolData {
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
    pool_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataSesv2DedicatedIpPool_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSesv2DedicatedIpPoolData>,
}

#[derive(Clone)]
pub struct DataSesv2DedicatedIpPool(std::rc::Rc<DataSesv2DedicatedIpPool_>);

impl DataSesv2DedicatedIpPool {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dedicated_ips` after provisioning.\n"]
    pub fn dedicated_ips(&self) -> terrars::ListRef<DataSesv2DedicatedIpPoolDedicatedIpsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dedicated_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pool_name` after provisioning.\n"]
    pub fn pool_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_mode` after provisioning.\n"]
    pub fn scaling_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scaling_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataSesv2DedicatedIpPool {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSesv2DedicatedIpPool { }

impl terrars::ToListMappable for DataSesv2DedicatedIpPool {
    type O = terrars::ListRef<DataSesv2DedicatedIpPoolRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSesv2DedicatedIpPool_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_sesv2_dedicated_ip_pool".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSesv2DedicatedIpPool {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub pool_name: terrars::PrimField<std::string::String>,
}

impl BuildDataSesv2DedicatedIpPool {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSesv2DedicatedIpPool {
        let out = DataSesv2DedicatedIpPool(std::rc::Rc::new(DataSesv2DedicatedIpPool_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSesv2DedicatedIpPoolData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                pool_name: self.pool_name,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSesv2DedicatedIpPoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSesv2DedicatedIpPoolRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSesv2DedicatedIpPoolRef {
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

    #[doc = "Get a reference to the value of field `dedicated_ips` after provisioning.\n"]
    pub fn dedicated_ips(&self) -> terrars::ListRef<DataSesv2DedicatedIpPoolDedicatedIpsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dedicated_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pool_name` after provisioning.\n"]
    pub fn pool_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pool_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_mode` after provisioning.\n"]
    pub fn scaling_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scaling_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSesv2DedicatedIpPoolDedicatedIpsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warmup_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    warmup_status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSesv2DedicatedIpPoolDedicatedIpsEl {
    #[doc = "Set the field `ip`.\n"]
    pub fn set_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `warmup_percentage`.\n"]
    pub fn set_warmup_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.warmup_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `warmup_status`.\n"]
    pub fn set_warmup_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.warmup_status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSesv2DedicatedIpPoolDedicatedIpsEl {
    type O = terrars::BlockAssignable<DataSesv2DedicatedIpPoolDedicatedIpsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSesv2DedicatedIpPoolDedicatedIpsEl {}

impl BuildDataSesv2DedicatedIpPoolDedicatedIpsEl {
    pub fn build(self) -> DataSesv2DedicatedIpPoolDedicatedIpsEl {
        DataSesv2DedicatedIpPoolDedicatedIpsEl {
            ip: core::default::Default::default(),
            warmup_percentage: core::default::Default::default(),
            warmup_status: core::default::Default::default(),
        }
    }
}

pub struct DataSesv2DedicatedIpPoolDedicatedIpsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSesv2DedicatedIpPoolDedicatedIpsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSesv2DedicatedIpPoolDedicatedIpsElRef {
        DataSesv2DedicatedIpPoolDedicatedIpsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSesv2DedicatedIpPoolDedicatedIpsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `warmup_percentage` after provisioning.\n"]
    pub fn warmup_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warmup_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `warmup_status` after provisioning.\n"]
    pub fn warmup_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warmup_status", self.base))
    }
}
