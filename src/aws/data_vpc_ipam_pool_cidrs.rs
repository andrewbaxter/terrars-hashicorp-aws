use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataVpcIpamPoolCidrsData {
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
    ipam_pool_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataVpcIpamPoolCidrsFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataVpcIpamPoolCidrsTimeoutsEl>,
    dynamic: DataVpcIpamPoolCidrsDynamic,
}

struct DataVpcIpamPoolCidrs_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVpcIpamPoolCidrsData>,
}

#[derive(Clone)]
pub struct DataVpcIpamPoolCidrs(std::rc::Rc<DataVpcIpamPoolCidrs_>);

impl DataVpcIpamPoolCidrs {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::BlockAssignable<DataVpcIpamPoolCidrsFilterEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataVpcIpamPoolCidrsTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pool_cidrs` after provisioning.\n"]
    pub fn ipam_pool_cidrs(&self) -> terrars::SetRef<DataVpcIpamPoolCidrsIpamPoolCidrsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipam_pool_cidrs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pool_id` after provisioning.\n"]
    pub fn ipam_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipam_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataVpcIpamPoolCidrsTimeoutsElRef {
        DataVpcIpamPoolCidrsTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataVpcIpamPoolCidrs {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataVpcIpamPoolCidrs { }

impl terrars::ToListMappable for DataVpcIpamPoolCidrs {
    type O = terrars::ListRef<DataVpcIpamPoolCidrsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataVpcIpamPoolCidrs_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_vpc_ipam_pool_cidrs".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVpcIpamPoolCidrs {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub ipam_pool_id: terrars::PrimField<std::string::String>,
}

impl BuildDataVpcIpamPoolCidrs {
    pub fn build(self, stack: &mut terrars::Stack) -> DataVpcIpamPoolCidrs {
        let out = DataVpcIpamPoolCidrs(std::rc::Rc::new(DataVpcIpamPoolCidrs_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataVpcIpamPoolCidrsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                ipam_pool_id: self.ipam_pool_id,
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataVpcIpamPoolCidrsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcIpamPoolCidrsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVpcIpamPoolCidrsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pool_cidrs` after provisioning.\n"]
    pub fn ipam_pool_cidrs(&self) -> terrars::SetRef<DataVpcIpamPoolCidrsIpamPoolCidrsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipam_pool_cidrs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipam_pool_id` after provisioning.\n"]
    pub fn ipam_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipam_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataVpcIpamPoolCidrsTimeoutsElRef {
        DataVpcIpamPoolCidrsTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataVpcIpamPoolCidrsIpamPoolCidrsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcIpamPoolCidrsIpamPoolCidrsEl {
    #[doc = "Set the field `cidr`.\n"]
    pub fn set_cidr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcIpamPoolCidrsIpamPoolCidrsEl {
    type O = terrars::BlockAssignable<DataVpcIpamPoolCidrsIpamPoolCidrsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcIpamPoolCidrsIpamPoolCidrsEl {}

impl BuildDataVpcIpamPoolCidrsIpamPoolCidrsEl {
    pub fn build(self) -> DataVpcIpamPoolCidrsIpamPoolCidrsEl {
        DataVpcIpamPoolCidrsIpamPoolCidrsEl {
            cidr: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct DataVpcIpamPoolCidrsIpamPoolCidrsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcIpamPoolCidrsIpamPoolCidrsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcIpamPoolCidrsIpamPoolCidrsElRef {
        DataVpcIpamPoolCidrsIpamPoolCidrsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcIpamPoolCidrsIpamPoolCidrsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcIpamPoolCidrsFilterEl {
    name: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataVpcIpamPoolCidrsFilterEl { }

impl terrars::ToListMappable for DataVpcIpamPoolCidrsFilterEl {
    type O = terrars::BlockAssignable<DataVpcIpamPoolCidrsFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcIpamPoolCidrsFilterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataVpcIpamPoolCidrsFilterEl {
    pub fn build(self) -> DataVpcIpamPoolCidrsFilterEl {
        DataVpcIpamPoolCidrsFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataVpcIpamPoolCidrsFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcIpamPoolCidrsFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcIpamPoolCidrsFilterElRef {
        DataVpcIpamPoolCidrsFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcIpamPoolCidrsFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcIpamPoolCidrsTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcIpamPoolCidrsTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcIpamPoolCidrsTimeoutsEl {
    type O = terrars::BlockAssignable<DataVpcIpamPoolCidrsTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcIpamPoolCidrsTimeoutsEl {}

impl BuildDataVpcIpamPoolCidrsTimeoutsEl {
    pub fn build(self) -> DataVpcIpamPoolCidrsTimeoutsEl {
        DataVpcIpamPoolCidrsTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataVpcIpamPoolCidrsTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcIpamPoolCidrsTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcIpamPoolCidrsTimeoutsElRef {
        DataVpcIpamPoolCidrsTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcIpamPoolCidrsTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataVpcIpamPoolCidrsDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataVpcIpamPoolCidrsFilterEl>>,
}
